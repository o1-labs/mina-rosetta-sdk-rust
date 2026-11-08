use super::Metadata;
use serde::{Deserialize, Serialize};

/// A shard of a network. Mina does not use it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SubNetworkIdentifier {
    pub network: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
