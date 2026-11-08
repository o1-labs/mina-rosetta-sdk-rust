use crate::models::Metadata;
use serde::{Deserialize, Serialize};

/// The body of `/network/list`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MetadataRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
