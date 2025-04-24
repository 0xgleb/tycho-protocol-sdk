use substreams::store::{StoreAddBigInt, StoreNew};
use tycho_substreams::models::BlockBalanceDeltas;

#[substreams::handlers::store]
pub fn store_balances(deltas: BlockBalanceDeltas, store: StoreAddBigInt) {
    tycho_substreams::balances::store_balance_changes(deltas, store);
}
