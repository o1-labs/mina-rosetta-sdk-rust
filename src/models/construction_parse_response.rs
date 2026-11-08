use super::{AccountIdentifier, Metadata, Operation};
use serde::{Deserialize, Serialize};

/// `/construction/parse`: the operations of a transaction and, when it is
/// signed, its signers. `signers` is the old field that
/// `account_identifier_signers` replaced in Rosetta 1.4.4.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConstructionParseResponse {
    pub operations: Vec<Operation>,
    #[serde(default)]
    pub signers: Vec<String>,
    #[serde(default)]
    pub account_identifier_signers: Vec<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
