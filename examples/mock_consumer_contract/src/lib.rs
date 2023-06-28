#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol, Vec};

mod std_reference {
    soroban_sdk::contractimport!(file = "../../dist/std_reference.wasm");
}

pub trait StandardReferenceTrait {
    fn set_std_reference_address(env: Env, std_reference_address: Address);
    fn get_price_of(env: Env, symbol_pairs: Vec<(Symbol, Symbol)>) -> u32;
}

pub struct MockConsumerContract;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub(crate) enum DataKey {
    StdReferenceAddress,
}

#[contractimpl]
impl MockConsumerContract {
    pub fn set_std_reference_address(env: Env, std_reference_address: Address) {
        env.storage()
            .set(&DataKey::StdReferenceAddress, &std_reference_address);
    }

    pub fn get_price_of(env: Env, symbol_pair: (Symbol, Symbol)) -> u128 {
        let addr = env
            .storage()
            .get_unchecked(&DataKey::StdReferenceAddress)
            .unwrap();
        let client = std_reference::Client::new(&env, &addr);
        client
            .get_reference_data(&Vec::from_array(&env, [symbol_pair]))
            .get_unchecked(0)
            .unwrap()
            .rate
    }
}
