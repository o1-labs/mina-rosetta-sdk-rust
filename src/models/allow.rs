use super::{ApiError, Case, OperationStatus};
use serde::{Deserialize, Serialize};

/// What the server supports: statuses, operation types and errors.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Allow {
    pub operation_statuses: Vec<OperationStatus>,
    pub operation_types: Vec<String>,
    pub errors: Vec<ApiError>,
    pub historical_balance_lookup: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_start_index: Option<i64>,
    #[serde(default)]
    pub call_methods: Vec<String>,
    #[serde(default)]
    pub balance_exemptions: Vec<serde_json::Value>,
    #[serde(default)]
    pub mempool_coins: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_hash_case: Option<Case>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash_case: Option<Case>,
}
