use super::TransactionSearch;
use crate::models::NetworkIdentifier;
use serde::{Deserialize, Serialize};

/// The body of `/search/transactions`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchTransactionsRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(flatten)]
    pub search: TransactionSearch,
}
