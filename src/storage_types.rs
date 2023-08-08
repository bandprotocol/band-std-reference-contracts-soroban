use soroban_sdk::{contracttype, Address, Symbol};

pub(crate) const TEMPORARY_BUMP_AMOUNT: u32 = 17280; // 1 day

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Admin,
    Relayer(Address),
    RefData(Symbol),
}
