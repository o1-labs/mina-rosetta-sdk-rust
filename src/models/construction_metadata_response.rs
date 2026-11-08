use super::{Amount, Metadata};
use serde::{Deserialize, Serialize};

/// `/construction/metadata`: what `/construction/payloads` needs, such as
/// the sender's nonce, and the suggested fee.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConstructionMetadataResponse {
    pub metadata: Metadata,
    #[serde(default)]
    pub suggested_fee: Vec<Amount>,
}
