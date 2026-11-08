use serde::{Deserialize, Serialize};

/// How a Mina balance splits: the part a vesting schedule still locks and
/// the part the account can spend, in nanomina.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BalanceMetadata {
    pub locked_balance: u64,
    pub liquid_balance: u64,
    pub total_balance: u64,
}
