use super::{BlockIdentifier, BlockMetadata, Transaction};
use serde::{Deserialize, Serialize};

/// A block and its transactions. `timestamp` is in milliseconds since the
/// Unix epoch.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Block {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>,
}
