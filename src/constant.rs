use soroban_sdk::contracterror;

pub(crate) const E9: u64 = 1_000_000_000_000_000_000;
pub(crate) const E18: u64 = 1_000_000_000_000_000_000;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StandardReferenceError {
    ParseError = 1,
    NotARelayerError = 2,
    InvalidSymbolRateError = 3,
    NoRefDataError = 4,
    InvalidRefDataError = 5,
    SymbolDoesNotExist = 6,
    DivisionError = 7,
    InvalidSymbolPairError = 8,
    UnknownError = 127,
}
