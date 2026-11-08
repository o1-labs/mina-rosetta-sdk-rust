use crate::models::{Metadata, NetworkIdentifier, PublicKey};
use serde::{Deserialize, Serialize};

/// The body of `/construction/derive`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionDeriveRequest {
    pub network_identifier: NetworkIdentifier,
    pub public_key: PublicKey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
