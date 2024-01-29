use soroban_sdk::contracttype;

use crate::constant::{StandardReferenceError, E18};
use crate::storage::ref_data::RefDatum;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct ReferenceDatum {
    // Pair rate e.g. rate of BTC/USD
    pub rate: u128,
    // Unix time of when the base asset was last updated. e.g. Last update time of BTC in Unix time
    pub last_updated_base: u64,
    // Unix time of when the quote asset was last updated. e.g. Last update time of USD in Unix time
    pub last_updated_quote: u64,
}

impl ReferenceDatum {
    pub fn new(rate: u128, last_updated_base: u64, last_updated_quote: u64) -> Self {
        if rate == 0 {
            panic!("rate cannot be zero")
        }

        ReferenceDatum {
            rate,
            last_updated_base,
            last_updated_quote,
        }
    }

    pub fn from_ref_datum(base: RefDatum, quote: RefDatum) -> Result<Self, StandardReferenceError> {
        let rate = (base.rate as u128)
            .checked_mul(E18 as u128)
            .ok_or(StandardReferenceError::ArithmeticError)?
            .checked_div(quote.rate as u128)
            .ok_or(StandardReferenceError::ArithmeticError)?;

        Ok(ReferenceDatum {
            rate,
            last_updated_base: base.resolve_time,
            last_updated_quote: quote.resolve_time,
        })
    }
}
