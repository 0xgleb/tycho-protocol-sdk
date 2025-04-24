use substreams::store::{StoreNew, StoreSet, StoreSetString};
use substreams_helper::hex::Hexable;

use crate::get_vault_balance_key;
use crate::pb::raindex::orderbook::events::orderbook_event::{self as event, AddOrderV2};
use crate::pb::raindex::orderbook::{events::OrderbookEvent, Events as RaindexEvents};
use crate::pb::raindex::orderbook::{Io, OrderV3};

#[substreams::handlers::store]
pub fn store_vault_orders(raindex_events: RaindexEvents, store: StoreSetString) {
    raindex_events
        .orderbook_events
        .iter()
        .for_each(|event| {
            if let OrderbookEvent {
                event:
                    Some(event::Event::AddOrder(AddOrderV2 { order_hash, order: Some(order), .. })),
                ..
            } = event
            {
                let OrderV3 { owner, valid_inputs, valid_outputs, .. } = order;

                let vaults = valid_inputs
                    .into_iter()
                    .chain(valid_outputs)
                    .map(|Io { token, vault_id }| get_vault_balance_key(&owner, token, vault_id));

                for vault in vaults {
                    // TODO: Handle cases where the same vault is used for multiple orders
                    store.set(0, &vault, &order_hash.to_hex());
                }
            }
        });
}
