use crate::models::{AccountIdentifier, Currency, NetworkIdentifier, PartialBlockIdentifier};
use serde::{Deserialize, Serialize};

/// The body of `/account/balance`. Without `block_identifier` the balance
/// is the one at the tip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountBalanceRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<PartialBlockIdentifier>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub currencies: Vec<Currency>,
}
