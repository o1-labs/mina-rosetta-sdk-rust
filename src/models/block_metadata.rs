use serde::{Deserialize, Serialize};

/// What Mina adds to a block: the public key of its creator.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlockMetadata {
    pub creator: String,
}
