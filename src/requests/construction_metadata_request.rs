use crate::models::{Metadata, NetworkIdentifier, PublicKey};
use serde::{Deserialize, Serialize};

/// The body of `/construction/metadata`: the options
/// `/construction/preprocess` returned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionMetadataRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKey>,
}
