use ethabi::ethereum_types::Address;
use std::str::FromStr;
use substreams::store::{StoreGet, StoreGetString};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;
use tycho_substreams::models::{BalanceDelta, BlockBalanceDeltas};

use crate::abi::raindex_orderbook::events as abi;
use crate::get_vault_balance_key;

#[substreams::handlers::map]
pub fn map_relative_balances(
    params: String,
    block: eth::Block,
    order_vaults_store: StoreGetString,
) -> Result<BlockBalanceDeltas, substreams::errors::Error> {
    let orderbook_address = Address::from_str(params.as_str())?;

    let balance_deltas: Vec<BalanceDelta> = block
        .logs()
        .filter(|log| {
            log.address().to_vec() == orderbook_address.0 && log.receipt.transaction.status == 1
        })
        .flat_map(|log| {
            let mut deltas = Vec::new();

            if let Some(event) = abi::Deposit::match_and_decode(log) {
                let abi::Deposit { sender: owner, token, vault_id, amount } = event;

                let vault_key = get_vault_balance_key(&owner, &token, &vault_id.to_bytes_be().1);

                if let Some(order_hash) = order_vaults_store.get_last(&vault_key) {
                    let order_hash = hex::decode(
                        order_hash
                            .strip_prefix("0x")
                            .unwrap_or(&order_hash),
                    )
                    .unwrap(); // TODO: remove unwrap

                    deltas.push(BalanceDelta {
                        ord: log.ordinal(),
                        tx: Some(log.receipt.transaction.into()),
                        token: token.to_vec(),
                        delta: amount.to_signed_bytes_be(),
                        component_id: order_hash,
                    });
                }
            } else if let Some(event) = abi::Withdraw::match_and_decode(log) {
                let abi::Withdraw { sender, token, vault_id, amount, target_amount: _ } = event;

                let vault_key = get_vault_balance_key(&sender, &token, &vault_id.to_bytes_be().1);

                if let Some(order_hash) = order_vaults_store.get_last(&vault_key) {
                    let order_hash = hex::decode(order_hash).unwrap(); // TODO: remove unwrap

                    deltas.push(BalanceDelta {
                        ord: log.ordinal(),
                        tx: Some(log.receipt.transaction.into()),
                        token: token.to_vec(),
                        delta: amount.neg().to_signed_bytes_be(),
                        component_id: order_hash,
                    });
                }
            }
            // TODO: handle TakeOrderV2 and ClearV2

            deltas
        })
        .collect::<Vec<_>>();

    Ok(BlockBalanceDeltas { balance_deltas })
}
