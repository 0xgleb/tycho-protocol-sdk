use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};
use substreams_helper::hex::Hexable;

use crate::pb::raindex::orderbook::events::orderbook_event::{self as event, Deposit};
use crate::pb::raindex::orderbook::{events::OrderbookEvent, Events as RaindexEvents};

#[substreams::handlers::store]
pub fn store_vaults_balances(raindex_events: RaindexEvents, store: StoreAddBigInt) {
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
                // TODO: qualify by orderbook address
                let key = format!(
                    "{owner}:{token}:{vault_id}",
                    owner = owner.to_hex(),
                    token = token.to_hex(),
                    vault_id = vault_id.to_hex(),
                );

                let amount = BigInt::from_unsigned_bytes_be(amount);

                store.add(*log_ordinal, key, amount);
            }
        });
}
