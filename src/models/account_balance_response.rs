use super::{AccountBalanceMetadata, Balance, BlockIdentifier};
use serde::{Deserialize, Serialize};

/// `/account/balance`: the balances of an account at a block.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    pub block_identifier: BlockIdentifier,
    pub balances: Vec<Balance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AccountBalanceMetadata>,
}

impl AccountBalanceResponse {
    /// The balance: Mina returns exactly one, for the asked token.
    pub fn balance(&self) -> Option<&Balance> {
        self.balances.first()
    }

    /// The account's nonce at the block.
    pub fn nonce(&self) -> Option<u32> {
        self.metadata.as_ref().map(|m| m.nonce)
    }
}
