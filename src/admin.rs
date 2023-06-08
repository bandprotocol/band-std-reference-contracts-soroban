use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn read_admin(env: &Env) -> Address {
    // This admin key has been set by owner and not expect to fail
    env.storage().get_unchecked(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin_addr: &Address) {
    env.storage().set(&DataKey::Admin, admin_addr);
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().has(&DataKey::Admin)
}
