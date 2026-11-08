use super::{BalanceMetadata, Currency};
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

/// A balance of `/account/balance`: the total, in the currency's smallest
/// unit, and how it splits into locked and liquid.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Balance {
    pub value: String,
    pub currency: Currency,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BalanceMetadata>,
}

impl Balance {
    /// The total balance as a number.
    pub fn total(&self) -> Result<u64, ParseIntError> {
        self.value.parse()
    }
}
