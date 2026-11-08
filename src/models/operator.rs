use serde::{Deserialize, Serialize};

/// How `/search/transactions` combines its conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    #[default]
    And,
    Or,
}
