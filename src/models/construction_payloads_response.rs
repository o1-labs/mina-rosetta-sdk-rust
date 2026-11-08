use super::SigningPayload;
use serde::{Deserialize, Serialize};

/// `/construction/payloads`: the unsigned transaction and what to sign.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConstructionPayloadsResponse {
    pub unsigned_transaction: String,
    pub payloads: Vec<SigningPayload>,
}
