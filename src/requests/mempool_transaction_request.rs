use crate::models::{NetworkIdentifier, TransactionIdentifier};
use serde::{Deserialize, Serialize};

/// The body of `/mempool/transaction`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolTransactionRequest {
    pub network_identifier: NetworkIdentifier,
    pub transaction_identifier: TransactionIdentifier,
}
