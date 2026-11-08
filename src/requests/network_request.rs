use crate::models::{Metadata, NetworkIdentifier};
use serde::{Deserialize, Serialize};

/// The body of `/network/status`, `/network/options` and `/mempool`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
