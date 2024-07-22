use crate::{abi, pool_factories};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use substreams::{
    hex,
    pb::substreams::StoreDeltas,
    store::{StoreAdd, StoreAddBigInt, StoreAddInt64, StoreGet, StoreGetInt64, StoreNew},
};
use substreams_ethereum::{pb::eth, Event};
use tycho_substreams::{
    balances::aggregate_balances_changes, contract::extract_contract_changes, prelude::*,
};

const VAULT_ADDRESS: &[u8] = &hex!("BA12222222228d8Ba445958a75a0704d566BF2C8");

#[substreams::handlers::map]
pub fn map_components(block: eth::v2::Block) -> Result<BlockTransactionProtocolComponents> {
    // Gather contract changes by indexing `PoolCreated` events and analysing the `Create` call
    // We store these as a hashmap by tx hash since we need to agg by tx hash later
    Ok(BlockTransactionProtocolComponents {
        tx_components: block
            .transactions()
            .filter_map(|tx| {
                let components = tx
                    .logs_with_calls()
                    .filter_map(|(log, call)| {
                        pool_factories::address_map(
                            call.call.address.as_slice(),
                            log,
                            call.call,
                            &(tx.into()),
                        )
                    })
                    .collect::<Vec<_>>();

                if !components.is_empty() {
                    Some(TransactionProtocolComponents { tx: Some(tx.into()), components })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
    })
}

/// Simply stores the `ProtocolComponent`s with the pool id as the key
#[substreams::handlers::store]
pub fn store_components(map: BlockTransactionProtocolComponents, store: StoreAddInt64) {
    store.add_many(
        0,
        &map.tx_components
            .iter()
            .flat_map(|tx_components| &tx_components.components)
            .map(|component| format!("pool:{0}", component.id))
            .collect::<Vec<_>>(),
        1,
    );
}

/// Since the `PoolBalanceChanged` and `Swap` events administer only deltas, we need to leverage a
/// map and a  store to be able to tally up final balances for tokens in a pool.
#[substreams::handlers::map]
pub fn map_relative_balances(
    block: eth::v2::Block,
    store: StoreGetInt64,
) -> Result<BlockBalanceDeltas, anyhow::Error> {
    let balance_deltas = block
        .logs()
        .filter(|log| log.address() == VAULT_ADDRESS)
        .flat_map(|vault_log| {
            let mut deltas = Vec::new();

            if let Some(ev) =
                abi::vault::events::PoolBalanceChanged::match_and_decode(vault_log.log)
            {
                let component_id = format!("0x{}", hex::encode(&ev.pool_id[..20]));

                if store
                    .get_last(format!("pool:{}", component_id))
                    .is_some()
                {
                    for (token, delta) in ev.tokens.iter().zip(ev.deltas.iter()) {
                        deltas.push(BalanceDelta {
                            ord: vault_log.ordinal(),
                            tx: Some(vault_log.receipt.transaction.into()),
                            token: token.to_vec(),
                            delta: delta.to_signed_bytes_be(),
                            component_id: component_id.as_bytes().to_vec(),
                        });
                    }
                }
            } else if let Some(ev) = abi::vault::events::Swap::match_and_decode(vault_log.log) {
                let component_id = format!("0x{}", hex::encode(&ev.pool_id[..20]));

                if store
                    .get_last(format!("pool:{}", component_id))
                    .is_some()
                {
                    deltas.extend_from_slice(&[
                        BalanceDelta {
                            ord: vault_log.ordinal(),
                            tx: Some(vault_log.receipt.transaction.into()),
                            token: ev.token_in.to_vec(),
                            delta: ev.amount_in.to_signed_bytes_be(),
                            component_id: component_id.as_bytes().to_vec(),
                        },
                        BalanceDelta {
                            ord: vault_log.ordinal(),
                            tx: Some(vault_log.receipt.transaction.into()),
                            token: ev.token_out.to_vec(),
                            delta: ev.amount_out.neg().to_signed_bytes_be(),
                            component_id: component_id.as_bytes().to_vec(),
                        },
                    ]);
                }
            }

            deltas
        })
        .collect::<Vec<_>>();

    Ok(BlockBalanceDeltas { balance_deltas })
}

/// It's significant to include both the `pool_id` and the `token_id` for each balance delta as the
///  store key to ensure that there's a unique balance being tallied for each.
#[substreams::handlers::store]
pub fn store_balances(deltas: BlockBalanceDeltas, store: StoreAddBigInt) {
    tycho_substreams::balances::store_balance_changes(deltas, store);
}

/// This is the main map that handles most of the indexing of this substream.
/// Every contract change is grouped by transaction index via the `transaction_changes`
///  map. Each block of code will extend the `TransactionChanges` struct with the
///  cooresponding changes (balance, component, contract), inserting a new one if it doesn't exist.
///  At the very end, the map can easily be sorted by index to ensure the final
/// `BlockChanges`  is ordered by transactions properly.
#[substreams::handlers::map]
pub fn map_protocol_changes(
    block: eth::v2::Block,
    grouped_components: BlockTransactionProtocolComponents,
    deltas: BlockBalanceDeltas,
    components_store: StoreGetInt64,
    balance_store: StoreDeltas, // Note, this map module is using the `deltas` mode for the store.
) -> Result<BlockChanges> {
    // We merge contract changes by transaction (identified by transaction index) making it easy to
    //  sort them at the very end.
    let mut transaction_changes: HashMap<_, TransactionChanges> = HashMap::new();

    // `ProtocolComponents` are gathered from `map_pools_created` which just need a bit of work to
    //   convert into `TransactionChanges`
    grouped_components
        .tx_components
        .iter()
        .for_each(|tx_component| {
            let tx = tx_component.tx.as_ref().unwrap();
            transaction_changes
                .entry(tx.index)
                .or_insert_with(|| TransactionChanges::new(tx))
                .component_changes
                .extend_from_slice(&tx_component.components);
        });

    block
        .transactions()
        .flat_map(|tx| {
            let components = tx
                .logs_with_calls()
                .filter(|(log, _)| log.address == VAULT_ADDRESS)
                .filter_map(|(log, _)| {
                    let registered = abi::vault::events::PoolRegistered::match_and_decode(log)?;
                    Some((
                        tx.clone(),
                        EntityChanges {
                            component_id: hex::encode(registered.pool_address),
                            attributes: vec![
                                Attribute {
                                    name: "pool_id".to_string(),
                                    value: format!("0x{}", hex::encode(registered.pool_id))
                                        .as_bytes()
                                        .to_vec(),
                                    change: ChangeType::Creation.into(),
                                },
                                Attribute {
                                    name: "balance_owner".to_string(),
                                    value: "0xBA12222222228d8Ba445958a75a0704d566BF2C8"
                                        .to_string()
                                        .as_bytes()
                                        .to_vec(),
                                    change: ChangeType::Creation.into(),
                                },
                            ],
                        },
                    ))
                });
            components
        })
        .for_each(|(tx, state_change)| {
            transaction_changes
                .entry(tx.index.into())
                .or_insert_with(|| TransactionChanges::new(&(&tx).into()))
                .entity_changes
                .push(state_change);
        });

    // Balance changes are gathered by the `StoreDelta` based on `PoolBalanceChanged` creating
    //  `BlockBalanceDeltas`. We essentially just process the changes that occurred to the `store`
    // this  block. Then, these balance changes are merged onto the existing map of tx contract
    // changes,  inserting a new one if it doesn't exist.
    aggregate_balances_changes(balance_store, deltas)
        .into_iter()
        .for_each(|(_, (tx, balances))| {
            transaction_changes
                .entry(tx.index)
                .or_insert_with(|| TransactionChanges::new(&tx))
                .balance_changes
                .extend(balances.into_values());
        });

    // Extract and insert any storage changes that happened for any of the components.
    extract_contract_changes(
        &block,
        |addr| {
            components_store
                .get_last(format!("pool:0x{0}", hex::encode(addr)))
                .is_some()
        },
        &mut transaction_changes,
    );

    // Process all `transaction_changes` for final output in the `BlockChanges`,
    //  sorted by transaction index (the key).
    Ok(BlockChanges {
        block: Some((&block).into()),
        changes: transaction_changes
            .drain()
            .sorted_unstable_by_key(|(index, _)| *index)
            .filter_map(|(_, change)| {
                if change.contract_changes.is_empty() &&
                    change.component_changes.is_empty() &&
                    change.balance_changes.is_empty() &&
                    change.entity_changes.is_empty()
                {
                    None
                } else {
                    Some(change)
                }
            })
            .collect::<Vec<_>>(),
    })
}
