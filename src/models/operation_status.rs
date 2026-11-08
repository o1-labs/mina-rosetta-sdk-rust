use serde::{Deserialize, Serialize};

/// An operation status the server can return, and whether it means success.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}
