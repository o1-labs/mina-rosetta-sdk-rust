use crate::models::NetworkIdentifier;
use serde::{Deserialize, Serialize};

/// The body of `/construction/parse`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionParseRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed: bool,
    pub transaction: String,
}
