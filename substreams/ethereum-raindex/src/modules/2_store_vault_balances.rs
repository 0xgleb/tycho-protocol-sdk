use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::get_vault_balance_key;

use crate::pb::raindex::orderbook::events::orderbook_event::{self as event, Deposit};
use crate::pb::raindex::orderbook::{events::OrderbookEvent, Events as RaindexEvents};

#[substreams::handlers::store]
pub fn store_vault_balances(raindex_events: RaindexEvents, store: StoreAddBigInt) {
    substreams::log::debug!(
        "store_vaults_balances called with {} raindex_events",
        raindex_events.orderbook_events.len()
    );

    raindex_events
        .orderbook_events
        .iter()
        .for_each(|event| {
            if let OrderbookEvent {
                log_ordinal,
                event: Some(event::Event::Deposit(deposit_event)),
            } = event
            {
                let Deposit { sender: owner, token, vault_id, amount } = deposit_event;

                let amount = BigInt::from_unsigned_bytes_be(amount);

                store.add(*log_ordinal, get_vault_balance_key(&owner, &token, &vault_id), amount);
            }
        });
}
