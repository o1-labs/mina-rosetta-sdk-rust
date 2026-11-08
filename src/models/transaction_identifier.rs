use serde::{Deserialize, Serialize};

/// A transaction by hash.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransactionIdentifier {
    pub hash: String,
}

impl TransactionIdentifier {
    pub fn new(hash: impl Into<String>) -> Self {
        TransactionIdentifier { hash: hash.into() }
    }
}
