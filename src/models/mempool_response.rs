use super::TransactionIdentifier;
use serde::{Deserialize, Serialize};

/// `/mempool`: the transactions in the daemon's pool.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MempoolResponse {
    pub transaction_identifiers: Vec<TransactionIdentifier>,
}
