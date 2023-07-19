use soroban_sdk::{Address, Env, Vec};

use crate::storage_types::DataKey;

// get_relayers returns the list of relaters
pub fn is_relayer(env: &Env, address: &Address) -> bool {
    env.storage().persistent().has(&DataKey::Relayer(address.clone()))
}

// add_relayers adds the given addresses to the relayers list
pub fn add_relayers(env: &Env, addrs: &Vec<Address>) {
    for addr in addrs.iter() {
        env.storage().persistent().set(&DataKey::Relayer(addr.clone()), &());
    }
}

// remove_relayers removes the given addresses from the relayers list
pub fn remove_relayers(env: &Env, addrs: &Vec<Address>) {
    for addr in addrs.iter() {
        env.storage().persistent().remove(&DataKey::Relayer(addr.clone()))
    }
}
