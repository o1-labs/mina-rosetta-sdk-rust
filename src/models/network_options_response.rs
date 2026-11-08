use super::{Allow, Version};
use serde::{Deserialize, Serialize};

/// `/network/options`: versions and what the server supports.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NetworkOptionsResponse {
    pub version: Version,
    pub allow: Allow,
}
