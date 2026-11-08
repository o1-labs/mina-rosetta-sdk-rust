use crate::models::{NetworkIdentifier, Signature};
use serde::{Deserialize, Serialize};

/// The body of `/construction/combine`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionCombineRequest {
    pub network_identifier: NetworkIdentifier,
    pub unsigned_transaction: String,
    pub signatures: Vec<Signature>,
}
