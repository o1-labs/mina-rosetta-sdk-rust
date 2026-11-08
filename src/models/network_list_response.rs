use super::NetworkIdentifier;
use serde::{Deserialize, Serialize};

/// `/network/list`: the networks the server serves.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}
