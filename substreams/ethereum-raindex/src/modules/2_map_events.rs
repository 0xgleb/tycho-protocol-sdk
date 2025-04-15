use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{self as eth};

use substreams_ethereum::Event;
use substreams_helper::{event_handler::EventHandler, hex::Hexable};

use crate::abi::raindex_orderbook::events as abi;
use crate::pb::raindex::orderbook::{
    events::orderbook_event as event, events::OrderbookEvent, EvaluableV3, Events as RaindexEvents,
    Io, OrderV3,
};

// use tycho_substreams::prelude::*;

#[substreams::handlers::map]
pub fn map_events(
    params: String,
    block: eth::Block,
) -> Result<RaindexEvents, substreams::errors::Error> {
    let orderbook_address = Address::from_str(params.as_str())?;

    let mut orderbook_events: Vec<OrderbookEvent> = block
        .logs()
        .filter(|log| log.address().to_vec() == orderbook_address.0)
        .filter_map(|log| {
            if let Some(event) = abi::Deposit::match_and_decode(log) {
                let abi::Deposit { sender, token, vault_id, amount } = event;

                let deposit_event = event::Deposit {
                    sender: sender.to_vec(),
                    token: token.to_vec(),
                    vault_id: vault_id.to_bytes_be().1,
                    amount: amount.to_bytes_be().1,
                };

                Some(OrderbookEvent {
                    log_ordinal: log.ordinal(),
                    event: Some(event::Event::Deposit(deposit_event)),
                })
            } else if let Some(event) = abi::Withdraw::match_and_decode(log) {
                let abi::Withdraw { sender, token, vault_id, amount, target_amount: _ } = event;

                let withdraw_event = event::Withdraw {
                    sender: sender.to_vec(),
                    token: token.to_vec(),
                    vault_id: vault_id.to_bytes_be().1,
                    amount: amount.to_bytes_be().1,
                };

                Some(OrderbookEvent {
                    log_ordinal: log.ordinal(),
                    event: Some(event::Event::Withdraw(withdraw_event)),
                })
            } else if let Some(event) = abi::AddOrderV2::match_and_decode(log) {
                let (owner, evaluable, valid_inputs, valid_outputs, _nonce) = event.order;

                let evaluable = Some(EvaluableV3 {
                    interpreter: evaluable.0,
                    store: evaluable.1,
                    // bytecode: evaluable.bytecode.to_vec(),
                });

                let valid_inputs: Vec<Io> = valid_inputs
                    .into_iter()
                    .map(|(token, _decimals, vault_id)| Io {
                        token: token.to_vec(),
                        // decimals,
                        vault_id: vault_id.to_bytes_be().1,
                    })
                    .collect();

                let valid_outputs: Vec<Io> = valid_outputs
                    .into_iter()
                    .map(|(token, _decimals, vault_id)| Io {
                        token: token.to_vec(),
                        // decimals,
                        vault_id: vault_id.to_bytes_be().1,
                    })
                    .collect();

                let add_order_event = event::AddOrderV2 {
                    sender: event.sender,
                    order_hash: event.order_hash.to_vec(),
                    order: Some(OrderV3 { owner, evaluable, valid_inputs, valid_outputs }),
                };

                Some(OrderbookEvent {
                    log_ordinal: log.ordinal(),
                    event: Some(event::Event::AddOrder(add_order_event)),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // get_events(&block, &mut orderbook_events, orderbook_address);

    orderbook_events.sort_unstable_by_key(|e| e.log_ordinal);

    Ok(RaindexEvents { orderbook_events })
}

// fn get_events(
//     block: &eth::Block,
//     orderbook_events: &mut Vec<OrderbookEvent>,
//     factory_address: &str,
// ) {
//     let mut event_handler = EventHandler::new(block);
//     event_handler.filter_by_address(vec![Address::from_str(factory_address).unwrap()]);

//     // Extract Deposit events from block logs
//     let mut on_deposit = |event: , _tx: &eth::TransactionTrace, log: &eth::Log| {
//     };

//     // Extract Withdraw events from block logs
//     let mut on_withdraw = |event: abi::Withdraw, _tx: &eth::TransactionTrace, log: &eth::Log| {
//     };

//     // Extract AddOrderV2 events from block logs
//     let mut on_order_added =
//         |event: abi::AddOrderV2, _tx: &eth::TransactionTrace, log: &eth::Log| {
//         };

//     event_handler.on::<abi::Deposit, _>(&mut on_deposit);
//     event_handler.on::<abi::Withdraw, _>(&mut on_withdraw);
//     event_handler.on::<abi::AddOrderV2, _>(&mut on_order_added);
//     event_handler.handle_events();
// }
