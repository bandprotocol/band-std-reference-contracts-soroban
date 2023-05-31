use soroban_sdk::{contracttype, Env, Symbol};

use crate::constant::{StandardReferenceError, E9};
use crate::storage_types::DataKey;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct RefData {
    pub rate: u64,
    pub resolve_time: u64,
    pub request_id: u64,
}

impl RefData {
    pub fn new(rate: u64, resolve_time: u64, request_id: u64) -> Self {
        Self {
            rate,
            resolve_time,
            request_id,
        }
    }

    pub fn usd(env: &Env) -> Self {
        Self::new(E9, env.ledger().timestamp(), 0)
    }

    pub fn set(&self, env: &Env, symbol: Symbol) -> &Self {
        // Do not allow USD to be overwritten
        if symbol != Symbol::new(env, "USD") {
            env.storage().set(&DataKey::RefData(symbol), self);
        }
        self
    }

    pub fn remove(env: &Env, symbol: Symbol) {
        env.storage().remove(&DataKey::RefData(symbol));
    }

    pub fn update(&mut self, rate: u64, resolve_time: u64, request_id: u64) -> &Self {
        if rate == 0 {
            panic!("rate cannot be zero")
        }

        if self.resolve_time < resolve_time {
            self.rate = rate;
            self.resolve_time = resolve_time;
            self.request_id = request_id;
        }

        self
    }

    pub fn unchecked_update(&mut self, rate: u64, resolve_time: u64, request_id: u64) -> &Self {
        if rate == 0 {
            panic!("rate cannot be zero")
        }

        self.rate = rate;
        self.resolve_time = resolve_time;
        self.request_id = request_id;

        self
    }
}

pub fn read_ref_data(env: &Env, symbol: Symbol) -> Result<RefData, StandardReferenceError> {
    if symbol == Symbol::new(&env, "USD") {
        return Ok(RefData::usd(&env));
    }

    if let Some(ref_data) = env.storage().get(&DataKey::RefData(symbol)) {
        ref_data.map_err(|_| StandardReferenceError::InvalidRefDataError)
    } else {
        Err(StandardReferenceError::NoRefDataError)
    }
}
