use super::BlockIdentifier;
use serde::{Deserialize, Serialize};

/// A block by height, by hash or by both. Neither means the tip.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PartialBlockIdentifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl PartialBlockIdentifier {
    /// The best tip.
    pub fn tip() -> Self {
        PartialBlockIdentifier::default()
    }

    /// The block at `index` on the best chain.
    pub fn at_index(index: i64) -> Self {
        PartialBlockIdentifier {
            index: Some(index),
            hash: None,
        }
    }

    /// The block with state hash `hash`.
    pub fn at_hash(hash: impl Into<String>) -> Self {
        PartialBlockIdentifier {
            index: None,
            hash: Some(hash.into()),
        }
    }
}

impl From<BlockIdentifier> for PartialBlockIdentifier {
    fn from(block: BlockIdentifier) -> Self {
        PartialBlockIdentifier {
            index: Some(block.index),
            hash: Some(block.hash),
        }
    }
}
