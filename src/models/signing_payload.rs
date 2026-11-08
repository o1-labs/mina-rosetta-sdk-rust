use super::{AccountIdentifier, SignatureType};
use serde::{Deserialize, Serialize};

/// Bytes an account must sign, hex encoded. `address` is the old field
/// that `account_identifier` replaced in Rosetta 1.4.4.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SigningPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    pub hex_bytes: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
}
