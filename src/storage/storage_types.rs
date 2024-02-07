use soroban_sdk::{contracttype, Address, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Admin,
    TTLConfig,
    Relayer(Address),
    RefData(Symbol),
}
