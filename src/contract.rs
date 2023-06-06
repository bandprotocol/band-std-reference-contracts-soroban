use soroban_sdk::{Address, BytesN, contractimpl, Env, Symbol, Vec};

use crate::admin::{has_admin, read_admin, write_admin};
use crate::constant::StandardReferenceError;
use crate::ref_data::{read_ref_data, RefData};
use crate::reference_data::ReferenceData;
use crate::relayer::{add_relayers, is_relayer, remove_relayers};

pub const VERSION: u32 = 1;

pub trait StandardReferenceTrait {
    fn init(env: Env, admin_addr: Address);
    fn version() -> u32;
    fn upgrade(env: Env, new_wasm_hash: BytesN<32>);
    fn current_admin(env: Env) -> Address;
    fn transfer_admin(env: Env, new_admin: Address);
    fn is_relayer(env: Env, address: Address) -> bool;
    fn add_relayers(env: Env, addresses: Vec<Address>);
    fn remove_relayers(env: Env, addresses: Vec<Address>);
    fn relay(
        env: Env,
        from: Address,
        symbol_rates: Vec<(Symbol, u64)>,
        resolve_time: u64,
        request_id: u64,
    );
    fn force_relay(
        env: Env,
        from: Address,
        symbol_rates: Vec<(Symbol, u64)>,
        resolve_time: u64,
        request_id: u64,
    );
    fn delist(env: Env, from: Address, symbols: Vec<Symbol>);
    fn get_reference_data(
        env: Env,
        symbol_pair: Vec<(Symbol, Symbol)>,
    ) -> Result<Vec<ReferenceData>, StandardReferenceError>;
}

pub struct StandardReference;

#[contractimpl]
impl StandardReferenceTrait for StandardReference {
    // Init initializes the contract with the given admin address where the admin address is also
    // added to the relayers list.
    fn init(env: Env, admin_addr: Address) {
        if has_admin(&env) {
            panic!("Already initialized");
        }

        write_admin(&env, &admin_addr);
        add_relayers(&env, &Vec::from_slice(&env, &[admin_addr]));
    }

    fn version() -> u32 {
        VERSION
    }

    // Upgrade upgrades the contract to the new wasm code at the given wasm hash.
    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        // Check that the caller is the admin
        let admin = read_admin(&env);
        admin.require_auth();

        env.update_current_contract_wasm(&new_wasm_hash)
    }

    fn current_admin(env: Env) -> Address {
        read_admin(&env)
    }

    // Transfers the admin to the new admin address and revokes relayer status from the old admin.
    fn transfer_admin(env: Env, new_admin: Address) {
        // Check that the caller is the admin
        let current_admin = read_admin(&env);
        current_admin.require_auth();

        // Transfer admin and revoke relayer status
        write_admin(&env, &new_admin);
        remove_relayers(&env, &Vec::from_array(&env, [new_admin.clone()]));
    }

    fn is_relayer(env: Env, address: Address) -> bool {
        is_relayer(&env, &address)
    }

    // Adds the given addresses to the relayers list.
    fn add_relayers(env: Env, addresses: Vec<Address>) {
        // Check that the caller is the admin
        read_admin(&env).require_auth();

        add_relayers(&env, &addresses);
    }

    // Removes the given addresses from the relayers list.
    fn remove_relayers(env: Env, addresses: Vec<Address>) {
        // Check that the caller is the admin
        read_admin(&env).require_auth();

        // Remove relayers
        remove_relayers(&env, &addresses);
    }

    // Relays the symbol rates to the contract. The caller must be a relayer.
    fn relay(
        env: Env,
        from: Address,
        symbol_rates: Vec<(Symbol, u64)>,
        resolve_time: u64,
        request_id: u64,
    ) {
        // Check that the caller is a relayer
        if !is_relayer(&env, &from) {
            panic!("Not a relayer");
        }
        from.require_auth();

        for symbol_rate in symbol_rates.iter() {
            if let Ok((symbol, rate)) = symbol_rate {
                if let Ok(mut ref_data) = read_ref_data(&env, symbol.clone()) {
                    ref_data
                        .update(rate, resolve_time, request_id)
                        .set(&env, symbol);
                } else {
                    RefData::new(rate, resolve_time, request_id).set(&env, symbol);
                }
            } else {
                panic!("Invalid symbol rate")
            }
        }
    }

    // Relays the symbol rates to the contract. The caller must be a relayer.
    fn force_relay(
        env: Env,
        from: Address,
        symbol_rates: Vec<(Symbol, u64)>,
        resolve_time: u64,
        request_id: u64,
    ) {
        // Check that the caller is a relayer
        if !is_relayer(&env, &from) {
            panic!("Not a relayer");
        }
        from.require_auth();

        for symbol_rate in symbol_rates.iter() {
            if let Ok((symbol, rate)) = symbol_rate {
                if let Ok(mut ref_data) = read_ref_data(&env, symbol.clone()) {
                    ref_data
                        .unchecked_update(rate, resolve_time, request_id)
                        .set(&env, symbol);
                } else {
                    RefData::new(rate, resolve_time, request_id).set(&env, symbol);
                }
            } else {
                panic!("Invalid symbol rate")
            }
        }
    }

    fn delist(env: Env, from: Address, symbols: Vec<Symbol>) {
        // Check that the caller is a relayer
        if !is_relayer(&env, &from) {
            panic!("Not a relayer");
        }
        from.require_auth();

        for symbol in symbols.iter() {
            if let Ok(symbol) = symbol {
                RefData::remove(&env, symbol);
            }
        }
    }

    fn get_reference_data(
        env: Env,
        symbol_pairs: Vec<(Symbol, Symbol)>,
    ) -> Result<Vec<ReferenceData>, StandardReferenceError> {
        let mut reference_data = Vec::new(&env);
        for symbol_pair in symbol_pairs.iter() {
            if let Ok((base, quote)) = symbol_pair {
                let base_ref = read_ref_data(&env, base);
                let quote_ref = read_ref_data(&env, quote);

                match (base_ref, quote_ref) {
                    (Ok(b), Ok(q)) => {
                        if let Some(r) = ReferenceData::from_ref_data(b, q) {
                            reference_data.push_back(r)
                        } else {
                            return Err(StandardReferenceError::DivisionError);
                        }
                    }
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            } else {
                return Err(StandardReferenceError::InvalidSymbolPairError);
            }
        }
        Ok(reference_data)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Mul;

    use soroban_sdk::{Address, Env, Symbol, testutils::Address as _, Vec};

    use crate::constant::{E9, StandardReferenceError};
    use crate::contract::StandardReference;
    use crate::reference_data::ReferenceData;
    use crate::StandardReferenceClient;

    fn register_contract(env: &Env) -> Address {
        env.register_contract(None, StandardReference {})
    }

    fn deploy_contract<'a>(
        env: &Env,
        admin: &Address,
        contract_id: &Address,
    ) -> StandardReferenceClient<'a> {
        let client = StandardReferenceClient::new(env, contract_id);
        client.init(admin);
        client
    }

    fn setup_relay(env: &Env, admin: &Address, contract: &StandardReferenceClient, time: &u64) {
        let symbol_rates = Vec::from_array(
            env,
            [
                (Symbol::short("AAA"), 1_000_000_000_000u64),
                (Symbol::short("BBB"), 9_999_000_000_000u64),
                (Symbol::short("CCC"), 1_234_000_000_000u64),
            ],
        );

        assert_eq!(true, contract.is_relayer(&admin));
        contract.relay(admin, &symbol_rates, &time, &1);
    }

    fn setup_overlap_relay(
        env: &Env,
        admin: &Address,
        contract: &StandardReferenceClient,
        time: &u64,
    ) {
        let symbol_rates = Vec::from_array(
            &env,
            [
                (Symbol::short("AAA"), 1_000_000_000u64),
                (Symbol::short("BBB"), 6_900_000_000_000u64),
                (Symbol::short("CCC"), 4_321_000_000_000u64),
            ],
        );
        assert_eq!(true, contract.is_relayer(&admin));
        contract.relay(&admin, &symbol_rates, &time, &2);
    }

    fn setup_force_relay(
        env: &Env,
        admin: &Address,
        contract: &StandardReferenceClient,
        time: &u64,
    ) {
        let symbol_rates = Vec::from_array(
            &env,
            [
                (Symbol::short("AAA"), 1_000_000_000u64),
                (Symbol::short("BBB"), 6_900_000_000_000u64),
                (Symbol::short("CCC"), 4_321_000_000_000u64),
            ],
        );
        assert_eq!(true, contract.is_relayer(&admin));
        contract.force_relay(&admin, &symbol_rates, &time, &2);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_reinit() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract_id = register_contract(&env);
        deploy_contract(&env, &admin, &contract_id);

        // Attempt to init the contract again, should panic
        deploy_contract(&env, &admin, &contract_id);
    }

    #[test]
    fn test_transfer_admin() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract = deploy_contract(&env, &admin, &register_contract(&env));

        // Attempt to transfer admin
        let new_admin = Address::random(&env);
        contract.transfer_admin(&new_admin);
        assert_eq!(contract.current_admin(), new_admin);
    }

    #[test]
    fn test_add_relayers() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract = deploy_contract(&env, &admin, &register_contract(&env));

        // Add relayers
        let relayer = Address::random(&env);
        contract.add_relayers(&Vec::from_array(&env, [relayer.clone()]));

        assert_eq!(true, contract.is_relayer(&admin));
        assert_eq!(true, contract.is_relayer(&relayer));
    }

    #[test]
    fn test_remove_relayers() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract = deploy_contract(&env, &admin, &register_contract(&env));

        // Test if
        contract.remove_relayers(&Vec::from_array(&env, [admin.clone()]));

        assert_eq!(false, contract.is_relayer(&admin));
    }

    #[test]
    fn test_relay() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract = deploy_contract(&env, &admin, &register_contract(&env));

        // Init relay
        setup_relay(&env, &admin, &contract, &1000u64);
        let query_pairs = Vec::from_array(
            &env,
            [
                (Symbol::short("AAA"), Symbol::short("USD")),
                (Symbol::short("BBB"), Symbol::short("USD")),
                (Symbol::short("CCC"), Symbol::short("USD")),
            ],
        );
        let actual = contract.get_reference_data(&query_pairs);
        let expected = Vec::from_array(
            &env,
            [
                ReferenceData::new(1_000_000_000_000u128.mul(E9 as u128), 1000u64, 0u64),
                ReferenceData::new(9_999_000_000_000u128.mul(E9 as u128), 1000u64, 0u64),
                ReferenceData::new(1_234_000_000_000u128.mul(E9 as u128), 1000u64, 0u64),
            ],
        );
        assert_eq!(expected, actual);

        // Relay with lower time, values should not change
        setup_overlap_relay(&env, &admin, &contract, &420u64);
        let actual = contract.get_reference_data(&query_pairs);
        assert_eq!(expected, actual);

        // Relay again with higher time, values should change
        setup_overlap_relay(&env, &admin, &contract, &1337u64);
        let actual = contract.get_reference_data(&query_pairs);
        let expected = Vec::from_array(
            &env,
            [
                ReferenceData::new(1_000_000_000u128.mul(E9 as u128), 1337u64, 0u64),
                ReferenceData::new(6_900_000_000_000u128.mul(E9 as u128), 1337u64, 0u64),
                ReferenceData::new(4_321_000_000_000u128.mul(E9 as u128), 1337u64, 0u64),
            ],
        );
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "Not a relayer")]
    fn test_unauthorized_relay() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract_id = register_contract(&env);
        let contract = deploy_contract(&env, &admin, &contract_id);

        // Attempt to with random user, should panic
        let symbol_rates = Vec::from_array(&env, [(Symbol::new(&env, "AAA"), 1000u64)]);
        contract.relay(&Address::random(&env), &symbol_rates, &1000, &1);
    }

    #[test]
    fn test_force_relay() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract = deploy_contract(&env, &admin, &register_contract(&env));

        // First relay
        setup_relay(&env, &admin, &contract, &1000u64);

        // Force relay, values should change regardless of time
        let query_pairs = Vec::from_array(
            &env,
            [
                (Symbol::short("AAA"), Symbol::short("USD")),
                (Symbol::short("BBB"), Symbol::short("USD")),
                (Symbol::short("CCC"), Symbol::short("USD")),
            ],
        );
        setup_force_relay(&env, &admin, &contract, &1u64);
        let actual = contract.get_reference_data(&query_pairs);
        let expected = Vec::from_array(
            &env,
            [
                ReferenceData::new(1_000_000_000u128.mul(E9 as u128), 1u64, 0u64),
                ReferenceData::new(6_900_000_000_000u128.mul(E9 as u128), 1u64, 0u64),
                ReferenceData::new(4_321_000_000_000u128.mul(E9 as u128), 1u64, 0u64),
            ],
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_delist() {
        // Setup environment
        let env = Env::default();
        env.mock_all_auths();

        // Init the contract
        let admin = Address::random(&env);
        let contract_id = register_contract(&env);
        let contract = deploy_contract(&env, &admin, &contract_id);

        // Init relay
        setup_relay(&env, &admin, &contract, &1000u64);

        // Delist AAA
        contract.delist(&admin, &Vec::from_array(&env, [Symbol::short("AAA")]));

        // Check if AAA is delisted
        let query = Vec::from_array(&env, [(Symbol::short("AAA"), Symbol::short("USD"))]);
        let actual = env
            .try_invoke_contract::<Vec<ReferenceData>, StandardReferenceError>(
                &contract_id,
                &Symbol::new(&env, "get_reference_data"),
                Vec::from_array(&env, [query.to_raw()]),
            )
            .err()
            .unwrap()
            .unwrap();
        assert_eq!(StandardReferenceError::NoRefDataError, actual);
    }
}
