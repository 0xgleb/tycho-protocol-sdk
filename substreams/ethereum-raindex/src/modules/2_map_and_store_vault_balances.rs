use std::str;

// use substreams::store::{StoreNew, StoreSetIfNotExists, StorePr};
// use tycho_substreams::models::BlockEntityChanges;

use crate::pb::raindex::orderbook::{Events as RaindexEvents, Vault};

#[substreams::handlers::store]
pub fn store_vaults_balances(raindex_events: RaindexEvents, store: StoreAddBigInt) {
    raindex_events.orderbook_events.iter().for_each(|event| {
        match event {
            OrderbookEvent::AddOrderV2(event) => {
                return;
            },
            OrderbookEvent::Deposit(event) => {
                let unqualified_vault_id = event.
            }
        }
    });
}
