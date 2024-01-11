use soroban_sdk::{Address, Env};

use crate::storage::storage_types::DataKey;

pub fn read_admin(env: &Env) -> Address {
    // This admin key has been set by owner and not expect to fail
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin_addr: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin_addr);
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}
