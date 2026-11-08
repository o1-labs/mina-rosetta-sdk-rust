use crate::models::{Metadata, NetworkIdentifier, Operation, PublicKey};
use serde::{Deserialize, Serialize};

/// The body of `/construction/payloads`: the operations and the metadata
/// `/construction/metadata` returned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionPayloadsRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKey>,
}
