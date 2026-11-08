use super::{BlockIdentifier, Transaction};
use serde::{Deserialize, Serialize};

/// A transaction found by `/search/transactions` and the block it is in.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlockTransaction {
    pub block_identifier: BlockIdentifier,
    pub transaction: Transaction,
}
