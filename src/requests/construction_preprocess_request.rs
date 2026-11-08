use crate::models::{Metadata, NetworkIdentifier, Operation};
use serde::{Deserialize, Serialize};

/// The body of `/construction/preprocess`. Mina reads `memo` and
/// `valid_until` from `metadata`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionPreprocessRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
