use soroban_sdk::{contracttype, Env, Symbol};

use crate::constant::{StandardReferenceError, E9};
use crate::storage_types::DataKey;
use crate::storage_types::TEMPORARY_BUMP_AMOUNT;

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
        let overridable = symbol != Symbol::new(env, "USD");
        let key = DataKey::RefData(symbol);

        if overridable {
            env.storage().temporary().set(&key, self);

            // Bump the temporary storage
            env.storage().temporary().bump(
                &key,
                TEMPORARY_BUMP_AMOUNT,
                TEMPORARY_BUMP_AMOUNT * 7,
            );
        }

        self
    }

    pub fn remove(env: &Env, symbol: Symbol) {
        env.storage().temporary().remove(&DataKey::RefData(symbol));
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

    let opt_ref_data: Option<RefData> = env.storage().temporary().get(&DataKey::RefData(symbol));

    if let Some(ref_data) = opt_ref_data {
        return Ok(ref_data)
    } else {
        Err(StandardReferenceError::NoRefDataError)
    }
}
