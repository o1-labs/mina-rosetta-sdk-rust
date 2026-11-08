use serde::{Deserialize, Serialize};

/// The position of an operation in its transaction.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OperationIdentifier {
    pub index: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_index: Option<i64>,
}

impl OperationIdentifier {
    pub fn new(index: i64) -> Self {
        OperationIdentifier {
            index,
            network_index: None,
        }
    }
}
