use super::{Direction, NetworkIdentifier, TransactionIdentifier};
use serde::{Deserialize, Serialize};

/// A transaction this one depends on, or that depends on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedTransaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    pub transaction_identifier: TransactionIdentifier,
    pub direction: Direction,
}
