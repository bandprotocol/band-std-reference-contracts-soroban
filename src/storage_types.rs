use soroban_sdk::{Address, contracttype, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub(crate) enum DataKey {
    Admin,
    Relayer(Address),
    RefData(Symbol),
}
