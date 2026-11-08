use crate::models::{NetworkIdentifier, PartialBlockIdentifier};
use serde::{Deserialize, Serialize};

/// The body of `/block`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: PartialBlockIdentifier,
}
