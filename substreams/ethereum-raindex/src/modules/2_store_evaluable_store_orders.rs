use substreams::store::{StoreNew, StoreSet, StoreSetString};
use substreams_helper::hex::Hexable;

use crate::pb::raindex::orderbook::events::orderbook_event::{self as event, AddOrderV2};
use crate::pb::raindex::orderbook::OrderV3;
use crate::pb::raindex::orderbook::{events::OrderbookEvent, Events as RaindexEvents};

#[substreams::handlers::store]
pub fn store_evaluable_store_orders(raindex_events: RaindexEvents, store: StoreSetString) {
    raindex_events
        .orderbook_events
        .iter()
        .for_each(|event| {
            if let OrderbookEvent {
                event:
                    Some(event::Event::AddOrder(AddOrderV2 {
                        order_hash,
                        order: Some(OrderV3 { evaluable: Some(evaluable), .. }),
                        ..
                    })),
                ..
            } = event
            {
                store.set(0, &evaluable.store.to_hex(), &order_hash.to_hex());
            }
        });
}
