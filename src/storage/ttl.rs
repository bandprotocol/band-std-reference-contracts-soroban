use soroban_sdk::{contracttype, Env, IntoVal, Val};

use crate::storage::storage_types::DataKey;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct TTLConfig {
    pub instance_threshold: u32,
    pub instance_ttl: u32,
    pub temporary_threshold: u32,
    pub temporary_ttl: u32,
}

impl TTLConfig {
    pub fn new(instance_threshold: u32, instance_ttl: u32, temporary_threshold: u32, temporary_ttl: u32) -> Self {
        Self {
            instance_threshold,
            instance_ttl,
            temporary_threshold,
            temporary_ttl,
        }
    }
}


pub fn write_ttl_config(env: &Env, instance_threshold: u32, instance_ttl: u32, temporary_threshold: u32, temporary_ttl: u32) {
    let max_allowable_ttl = env.storage().max_ttl();
    if instance_ttl >= max_allowable_ttl || temporary_ttl >= max_allowable_ttl {
        panic!("ttl is larger than maximum allowed ttl of {}", max_allowable_ttl)
    }

    // Hardcoded for now but should get state from network parameters
    if instance_threshold < 16 || temporary_threshold < 16 {
        panic!("ttl threshold is smaller than minimum allowed ttl of 16")
    }

    let config = TTLConfig::new(instance_threshold, instance_ttl, temporary_threshold, temporary_ttl);
    env.storage().instance().set(&DataKey::TTLConfig, &config);
}

pub fn has_ttl_config(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::TTLConfig)
}

pub fn read_instance_ttl_config(env: &Env) -> (u32, u32) {
    let config = read_ttl_config(env);
    (config.instance_threshold, config.instance_ttl)
}

pub fn read_temporary_ttl_config(env: &Env) -> (u32, u32) {
    let config = read_ttl_config(env);
    (config.temporary_threshold, config.temporary_ttl)
}

pub fn read_ttl_config(env: &Env) -> TTLConfig {
    if let Some(config) = env.storage().instance().get::<_, TTLConfig>(&DataKey::TTLConfig) {
        return config
    }
    panic!("TTLConfig not set")
}

pub fn bump_instance_ttl(env: &Env) {
    let (threshold, ttl) = read_instance_ttl_config(&env);
    env.storage().instance().extend_ttl(threshold, ttl);
}

pub fn bump_temporary_ttl<K>(env: &Env, key: &K)
    where K: IntoVal<Env, Val>,
{
    let (threshold, ttl) = read_temporary_ttl_config(&env);
    env.storage().temporary().extend_ttl(key, threshold, ttl);
}
