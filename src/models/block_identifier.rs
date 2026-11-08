use serde::{Deserialize, Serialize};

/// A block: its height (`index`) and its state hash.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlockIdentifier {
    pub index: i64,
    pub hash: String,
}
