use crate::models::{AccountIdentifier, Currency, Operator, TransactionIdentifier};
use serde::{Deserialize, Serialize};

/// The conditions of `/search/transactions`. Every condition is optional;
/// `operator` says whether all or any must hold.
///
/// ```
/// use mina_rosetta_sdk::TransactionSearch;
///
/// let search = TransactionSearch::default().address("B62q...").limit(10);
/// assert_eq!(search.limit, Some(10));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransactionSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_block: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl TransactionSearch {
    /// Transactions with an operation on any account of `address`.
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Transactions with an operation on `account`.
    pub fn account(mut self, account: impl Into<AccountIdentifier>) -> Self {
        self.account_identifier = Some(account.into());
        self
    }

    /// The transaction with hash `hash`.
    pub fn transaction(mut self, hash: impl Into<String>) -> Self {
        self.transaction_identifier = Some(TransactionIdentifier::new(hash));
        self
    }

    /// Transactions with an operation of type `operation_type`.
    pub fn operation_type(mut self, operation_type: impl Into<String>) -> Self {
        self.operation_type = Some(operation_type.into());
        self
    }

    /// Transactions with an operation of status `status`.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Transactions that succeeded, or that failed.
    pub fn success(mut self, success: bool) -> Self {
        self.success = Some(success);
        self
    }

    /// Whether all conditions must hold (`And`, the default) or any.
    pub fn operator(mut self, operator: Operator) -> Self {
        self.operator = Some(operator);
        self
    }

    /// Transactions in blocks up to height `max_block`. Paging with an
    /// offset is stable only with a fixed `max_block`.
    pub fn max_block(mut self, max_block: i64) -> Self {
        self.max_block = Some(max_block);
        self
    }

    /// Skip the first `offset` matches.
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Return at most `limit` matches.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
