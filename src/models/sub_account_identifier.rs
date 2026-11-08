use super::Metadata;
use serde::{Deserialize, Serialize};

/// A part of an account. Mina does not use it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SubAccountIdentifier {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
