use super::{AccountIdentifier, Metadata};
use serde::{Deserialize, Serialize};

/// `/construction/derive`: the account of a public key. `address` is the
/// old field that `account_identifier` replaced in Rosetta 1.4.4.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConstructionDeriveResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
