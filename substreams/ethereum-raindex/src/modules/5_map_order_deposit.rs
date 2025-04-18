use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetString};
use substreams_ethereum::pb::eth::v2::{self as eth};

use crate::get_vault_balance_key;
use crate::pb::raindex::orderbook::{
    events::orderbook_event::{self as event, Deposit},
    events::OrderbookEvent,
    Events as RaindexEvents,
};

use tycho_substreams::prelude::*;

#[substreams::handlers::map]
pub fn map_order_deposit(
    block: eth::Block,
    events: RaindexEvents,
    vault_balances_store: StoreGetBigInt,
    order_vaults_store: StoreGetString,
) -> Result<BlockEntityChanges, substreams::errors::Error> {
    substreams::log::debug!(
        "map_order_deposit called on block {} with {} events",
        block.number,
        events.orderbook_events.len()
    );

    let mut liquidity_updates: Vec<TransactionEntityChanges> = vec![];

    process_deposits(
        &block,
        &mut liquidity_updates,
        &events,
        vault_balances_store,
        order_vaults_store,
    )?;

    let block = if liquidity_updates.is_empty() { None } else { Some((&block).into()) };

    Ok(BlockEntityChanges { block, changes: liquidity_updates })
}

fn process_deposits(
    block: &eth::Block,
    liquidity_updates: &mut Vec<TransactionEntityChanges>,
    events: &RaindexEvents,
    vault_balances_store: StoreGetBigInt,
    order_vaults_store: StoreGetString,
) -> Result<(), substreams::errors::Error> {
    for orderbook_event in &events.orderbook_events {
        if let OrderbookEvent { log_ordinal, event: Some(event::Event::Deposit(deposit_event)) } =
            orderbook_event
        {
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
                                .any(|log| log.ordinal == *log_ordinal)
                        })
                        .unwrap_or(false)
                })
                .ok_or_else(|| {
                    substreams::errors::Error::msg(format!(
                        "Failed to find transaction for log ordinal {} in block {}",
                        log_ordinal, block.number
                    ))
                })?;

            let tycho_tx: Transaction = tx.into();

            let Deposit { sender: owner, token, vault_id, amount: _ } = deposit_event;

            // Get the vault key for this deposit
            let vault_key = get_vault_balance_key(owner, token, vault_id);

            // Find all orders that use this vault
            if let Some(order_hash) = order_vaults_store.get_last(&vault_key) {
                let balance = vault_balances_store
                    .get_last(&vault_key)
                    .unwrap_or(BigInt::from(0));

                let component_id = hex::decode(
                    order_hash
                        .strip_prefix("0x")
                        .unwrap_or(&order_hash),
                )
                .unwrap();

                liquidity_updates.push(TransactionEntityChanges {
                    tx: Some(tycho_tx.clone()),
                    entity_changes: vec![],
                    component_changes: vec![ProtocolComponent {
                        id: order_hash,
                        tokens: vec![token.clone()],
                        contracts: vec![],
                        static_att: vec![],
                        change: i32::from(ChangeType::Update),
                        protocol_type: Option::from(ProtocolType {
                            name: "raindex_order".to_string(),
                            financial_type: FinancialType::Swap.into(),
                            attribute_schema: vec![],
                            implementation_type: ImplementationType::Custom.into(),
                        }),
                        tx: Some(tycho_tx),
                    }],
                    balance_changes: vec![BalanceChange {
                        token: token.clone(),
                        balance: balance.to_signed_bytes_be(),
                        component_id,
                    }],
                });
            }
        }
    }

    Ok(())
}
