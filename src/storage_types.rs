use soroban_sdk::{contracttype, Address, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub(crate) enum DataKey {
    Admin,
    Relayer(Address),
    RefData(Symbol),
}
