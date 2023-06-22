use soroban_sdk::contracterror;

pub const E9: u64 = 1_000_000_000;
pub const E18: u64 = 1_000_000_000_000_000_000;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StandardReferenceError {
    NotInitializedError = 0,
    NoRefDataError = 1,
    InvalidRefDataError = 2,
    InvalidSymbolError = 3,
    InvalidSymbolPairError = 4,
    ArithmeticError = 5,
}
