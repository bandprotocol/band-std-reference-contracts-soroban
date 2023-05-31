use soroban_sdk::contracttype;

use crate::constant::E18;
use crate::ref_data::RefData;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct ReferenceData {
    // Pair rate e.g. rate of BTC/USD
    pub rate: u128,
    // Unix time of when the base asset was last updated. e.g. Last update time of BTC in Unix time
    pub last_updated_base: u64,
    // Unix time of when the quote asset was last updated. e.g. Last update time of USD in Unix time
    pub last_updated_quote: u64,
}

impl ReferenceData {
    pub fn new(rate: u128, last_updated_base: u64, last_updated_quote: u64) -> Self {
        ReferenceData {
            rate,
            last_updated_base,
            last_updated_quote,
        }
    }

    pub fn from_ref_data(base: RefData, quote: RefData) -> Option<Self> {
        let rate = (base.rate as u128).checked_mul(E18 as u128).unwrap().checked_div(quote.rate as u128);
        if rate.is_none() {
            return None;
        }

        Some(ReferenceData {
            rate: rate.unwrap(),
            last_updated_base: base.resolve_time,
            last_updated_quote: quote.resolve_time,
        })
    }
}
