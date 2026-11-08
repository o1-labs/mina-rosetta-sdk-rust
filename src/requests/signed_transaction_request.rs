use crate::models::NetworkIdentifier;
use serde::{Deserialize, Serialize};

/// The body of `/construction/hash` and `/construction/submit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTransactionRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}
