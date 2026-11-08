use super::Metadata;
use serde::{Deserialize, Serialize};

/// A peer of the node.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
