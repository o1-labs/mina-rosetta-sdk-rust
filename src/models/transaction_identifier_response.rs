use super::{Metadata, TransactionIdentifier};
use serde::{Deserialize, Serialize};

/// `/construction/hash` and `/construction/submit`: the transaction's hash.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransactionIdentifierResponse {
    pub transaction_identifier: TransactionIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
