use super::{AccountIdentifier, Amount, Metadata, OperationIdentifier, OperationType};
use serde::{Deserialize, Serialize};

/// One balance change of a transaction. `status` is set on chain
/// (`Success` or `Failed`) and must be empty when constructing.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Operation {
    pub operation_identifier: OperationIdentifier,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_operations: Vec<OperationIdentifier>,
    #[serde(rename = "type")]
    pub operation_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl Operation {
    /// The type, if it is one Mina defines.
    pub fn kind(&self) -> Option<OperationType> {
        self.operation_type.parse().ok()
    }
}
