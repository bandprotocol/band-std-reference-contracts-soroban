use soroban_sdk::{contracttype, Address, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Admin,
    MaxTTL,
    Relayer(Address),
    RefData(Symbol),
}
