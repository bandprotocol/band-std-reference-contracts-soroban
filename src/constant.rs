use soroban_sdk::contracterror;

pub(crate) const E9: u64 = 1_000_000_000_000_000_000;
pub(crate) const E18: u64 = 1_000_000_000_000_000_000;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StandardReferenceError {
    NoRefDataError = 1,
    InvalidRefDataError = 2,
    InvalidSymbolPairError = 3,
    DivisionError = 4,
}
