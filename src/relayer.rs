use soroban_sdk::{Address, Env, Vec};

use crate::storage_types::DataKey;

// get_relayers returns the list of relaters
pub fn is_relayer(env: &Env, address: &Address) -> bool {
    env.storage().has(&DataKey::Relayer(address.clone()))
}

// add_relayers adds the given addresses to the relayers list
pub fn add_relayers(env: &Env, addrs: &Vec<Address>) {
    for addr_res in addrs.iter() {
        if let Ok(addr) = addr_res {
            env.storage().set(&DataKey::Relayer(addr.clone()), &());
        }
    }
}

// remove_relayers removes the given addresses from the relayers list
pub fn remove_relayers(env: &Env, addrs: &Vec<Address>) {
    for addr_res in addrs.iter() {
        if let Ok(addr) = addr_res {
            env.storage().remove(&DataKey::Relayer(addr.clone()))
        }
    }
}
