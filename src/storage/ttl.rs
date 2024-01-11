use soroban_sdk::Env;

use crate::storage::storage_types::DataKey;

pub fn read_max_ttl(env: &Env) -> u32{
    env.storage().instance().get(&DataKey::MaxTTL).unwrap()
}

pub fn write_max_ttl(env: &Env, ttl: u32) {
    let max_allowable_ttl = env.storage().max_ttl();
    if ttl > max_allowable_ttl {
        panic!("ttl is larger than maximum allowed ttl {}", max_allowable_ttl)
    }
    env.storage().instance().set(&DataKey::MaxTTL, &ttl)
}

pub fn has_max_ttl(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::MaxTTL)
}

pub fn bump_instance_ttl_to_max(env: &Env) {
    let max_ttl = env.storage().max_ttl();
    env.storage().instance().extend_ttl(max_ttl, max_ttl);
}
