use serde::{Deserialize, Serialize};

/// How far the node is synced. Mina sets `stage` to the daemon's sync
/// status, such as `Synced` or `Bootstrap`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
}
