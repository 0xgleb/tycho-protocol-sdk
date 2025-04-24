mod abi;
mod modules;
mod pb;
// mod pool_factories;

// pub use modules::*;

use substreams_helper::hex::Hexable;

pub fn get_vault_balance_key(owner: &Vec<u8>, token: &Vec<u8>, vault_id: &Vec<u8>) -> String {
    format!(
        "{owner}:{token}:{vault_id}",
        owner = owner.to_hex(),
        token = token.to_hex(),
        vault_id = vault_id.to_hex(),
    )
}
