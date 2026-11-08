use super::BlockTransaction;
use serde::{Deserialize, Serialize};

/// `/search/transactions`: a page of the transactions that match, the
/// number that match in all, and the offset of the next page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchTransactionsResponse {
    pub transactions: Vec<BlockTransaction>,
    pub total_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
}
