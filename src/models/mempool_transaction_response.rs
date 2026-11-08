use super::{Metadata, Transaction};
use serde::{Deserialize, Serialize};

/// `/mempool/transaction`: a pooled transaction and its operations.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MempoolTransactionResponse {
    pub transaction: Transaction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
