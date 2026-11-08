use super::{AccountIdentifier, Metadata};
use serde::{Deserialize, Serialize};

/// `/construction/preprocess`: the options to pass to
/// `/construction/metadata`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConstructionPreprocessResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Metadata>,
    #[serde(default)]
    pub required_public_keys: Vec<AccountIdentifier>,
}
