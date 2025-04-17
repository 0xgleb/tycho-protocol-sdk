use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt};
use substreams_ethereum::pb::eth::v2::{self as eth};

use substreams_helper::hex::Hexable;

use crate::get_vault_balance_key;
use crate::pb::raindex::orderbook::{
    events::orderbook_event as event, events::OrderbookEvent, Events as RaindexEvents,
};

use tycho_substreams::prelude::*;

#[substreams::handlers::map]
pub fn map_order_added(
    block: eth::Block,
    events: RaindexEvents,
    store: StoreGetBigInt,
) -> Result<BlockEntityChanges, substreams::errors::Error> {
    substreams::log::debug!(
        "map_order_added called on block {} with {} events",
        block.number,
        events.orderbook_events.len()
    );

    let mut new_orders: Vec<TransactionEntityChanges> = vec![];

    get_new_orders(&block, &mut new_orders, &events, store)?;

    Ok(BlockEntityChanges { block: Some((&block).into()), changes: new_orders })
}

// Extract new orders from AddOrderV2 events
fn get_new_orders(
    block: &eth::Block,
    new_orders: &mut Vec<TransactionEntityChanges>,
    events: &RaindexEvents,
    store: StoreGetBigInt,
) -> Result<(), substreams::errors::Error> {
    for orderbook_event in &events.orderbook_events {
        if let OrderbookEvent {
            log_ordinal,
            event: Some(event::Event::AddOrder(add_order_event)),
        } = orderbook_event
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

            let order = add_order_event
                .order
                .as_ref()
                .ok_or_else(|| {
                    substreams::errors::Error::msg(format!(
                        "AddOrder event has no order data in block {}",
                        block.number
                    ))
                })?;

            let owner = &order.owner;
            let valid_inputs = &order.valid_inputs;
            let valid_outputs = &order.valid_outputs;

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

            let balance_changes = valid_inputs
                .iter()
                .chain(valid_outputs.iter())
                .map(|io| {
                    let balance = store
                        .get_last(get_vault_balance_key(owner, &io.token, &io.vault_id))
                        .unwrap_or(BigInt::from(0));

                    BalanceChange {
                        token: io.token.clone(),
                        balance: balance.to_signed_bytes_be(),
                        component_id: add_order_event.order_hash.clone(),
                    }
                })
                .collect::<Vec<_>>();

            let component_id = format!(
                "{owner}:{order}",
                owner = owner.to_hex(),
                order = add_order_event.order_hash.to_hex()
            );

            new_orders.push(TransactionEntityChanges {
                tx: Some(tycho_tx.clone()),
                entity_changes: vec![EntityChanges {
                    component_id: component_id.clone(),
                    attributes: vec![],
                }],
                component_changes: vec![ProtocolComponent {
                    id: component_id,
                    tokens,
                    contracts: vec![],
                    static_att: vec![],
                    change: i32::from(ChangeType::Creation),
                    protocol_type: Option::from(ProtocolType {
                        name: "raindex_order".to_string(),
                        financial_type: FinancialType::Swap.into(),
                        attribute_schema: vec![],
                        implementation_type: ImplementationType::Custom.into(),
                    }),
                    tx: Some(tycho_tx),
                }],
                balance_changes,
            });
        }
    }

    Ok(())
}
