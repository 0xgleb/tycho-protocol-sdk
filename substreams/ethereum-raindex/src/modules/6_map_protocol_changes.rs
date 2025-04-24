use itertools::Itertools;
use std::collections::HashMap;

use substreams::pb::substreams::StoreDeltas;
use substreams::store::{StoreGet, StoreGetString};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::hex::Hexable;
use tycho_substreams::balances::aggregate_balances_changes;
use tycho_substreams::contract::extract_contract_changes_builder;
use tycho_substreams::prelude::*;

use crate::pb::raindex::orderbook::{
    events::orderbook_event as event, events::OrderbookEvent, Events as RaindexEvents, OrderV3,
};

#[substreams::handlers::map]
pub fn map_protocol_changes(
    block: eth::Block,
    events: RaindexEvents,
    balance_deltas: BlockBalanceDeltas,
    // vault_balance_store: StoreGetBigInt,
    balance_store: StoreDeltas, // Note, this map module is using the `deltas` mode for the store.
    evaluable_store: StoreGetString,
) -> Result<BlockChanges, substreams::errors::Error> {
    substreams::log::debug!(
        "map_order_added called on block {} with {} events",
        block.number,
        events.orderbook_events.len()
    );

    let mut transaction_changes: HashMap<u64, TransactionChangesBuilder> = HashMap::new();

    get_added_orders(&block, &mut transaction_changes, &events)?;

    aggregate_balances_changes(balance_store, balance_deltas)
        .into_iter()
        .for_each(|(_, (tx, balances))| {
            let builder = transaction_changes
                .entry(tx.index)
                .or_insert_with(|| TransactionChangesBuilder::new(&tx));

            balances
                .values()
                .for_each(|token_balance_changes| {
                    token_balance_changes
                        .values()
                        .for_each(|balance_change| builder.add_balance_change(balance_change))
                });
        });

    extract_contract_changes_builder(
        &block,
        |addr| {
            evaluable_store
                .get_last(hex::encode(addr))
                .is_some()
        },
        &mut transaction_changes,
    );

    let changes = transaction_changes
        .drain()
        .sorted_unstable_by_key(|(index, _)| *index)
        .filter_map(|(_, builder)| builder.build())
        .collect::<Vec<_>>();
    let block = if changes.is_empty() { None } else { Some((&block).into()) };

    Ok(BlockChanges { block, changes })
}

// Extract new orders from AddOrderV2 events
fn get_added_orders(
    block: &eth::Block,
    transaction_changes: &mut HashMap<u64, TransactionChangesBuilder>,
    events: &RaindexEvents,
    // vault_balance_store: StoreGetBigInt,
) -> Result<(), substreams::errors::Error> {
    for orderbook_event in &events.orderbook_events {
        if let OrderbookEvent {
            log_ordinal,
            event: Some(event::Event::AddOrder(add_order_event)),
        } = orderbook_event
        {
            let tycho_tx = get_tx_by_log_ordinal(block, *log_ordinal)?;

            let builder = transaction_changes
                .entry(tycho_tx.index)
                .or_insert_with(|| TransactionChangesBuilder::new(&tycho_tx));

            let order = add_order_event
                .order
                .as_ref()
                .ok_or_else(|| {
                    substreams::errors::Error::msg(format!(
                        "AddOrder event has no order data in block {}",
                        block.number
                    ))
                })?;

            let OrderV3 { owner: _, evaluable, valid_inputs, valid_outputs } = order;

            let evaluable = evaluable.as_ref().ok_or_else(|| {
                substreams::errors::Error::msg(format!(
                    "AddOrder event has no evaluable data in block {}",
                    block.number
                ))
            })?;

            let input_tokens = valid_inputs
                .iter()
                .map(|io| io.token.clone())
                .collect::<Vec<_>>();

            let output_tokens = valid_outputs
                .iter()
                .map(|io| io.token.clone())
                .collect::<Vec<_>>();

            let tokens = input_tokens
                .iter()
                .chain(output_tokens.iter())
                .cloned()
                .collect::<std::collections::HashSet<_>>();

            let tokens: Vec<_> = tokens.into_iter().collect();

            // let balance_changes = valid_inputs
            //     .iter()
            //     .chain(valid_outputs.iter())
            //     .map(|io| {
            //         let balance = vault_balance_store
            //             .get_last(get_vault_balance_key(owner, &io.token, &io.vault_id))
            //             .unwrap_or(BigInt::from(0));

            //         BalanceChange {
            //             token: io.token.clone(),
            //             balance: balance.to_signed_bytes_be(),
            //             component_id: add_order_event.order_hash.clone(),
            //         }
            //     })
            //     .collect::<Vec<_>>();

            let component_id = add_order_event.order_hash.to_hex();

            let protocol_component = ProtocolComponent {
                id: component_id.clone(),
                tokens,
                contracts: vec![evaluable.interpreter.clone(), evaluable.store.clone()],
                static_att: vec![],
                change: i32::from(ChangeType::Creation),
                protocol_type: Some(ProtocolType {
                    name: "raindex_order".to_string(),
                    financial_type: FinancialType::Swap.into(),
                    attribute_schema: vec![],
                    implementation_type: ImplementationType::Custom.into(),
                }),
                tx: Some(tycho_tx),
            };

            builder.add_protocol_component(&protocol_component);
            // TODO: add attributes
            builder.add_entity_change(&EntityChanges { component_id, attributes: vec![] });
        }
    }

    Ok(())
}

fn get_tx_by_log_ordinal(
    block: &eth::Block,
    log_ordinal: u64,
) -> Result<Transaction, substreams::errors::Error> {
    let tx = block
        .transaction_traces
        .iter()
        .find(|tx| {
            tx.receipt
                .as_ref()
                .map(|receipt| {
                    receipt
                        .logs
                        .iter()
                        .any(|log| log.ordinal == log_ordinal)
                })
                .unwrap_or(false)
        })
        .ok_or_else(|| {
            substreams::errors::Error::msg(format!(
                "Failed to find transaction for log ordinal {} in block {}",
                log_ordinal, block.number
            ))
        })?;

    Ok(tx.into())
}
