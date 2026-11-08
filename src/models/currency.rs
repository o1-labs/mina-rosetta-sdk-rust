use super::TokenMetadata;
use serde::{Deserialize, Serialize};

/// The unit of an amount: MINA, or `MINA+` with the token id in the
/// metadata for any other token.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Currency {
    pub symbol: String,
    pub decimals: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TokenMetadata>,
}

impl Currency {
    /// MINA, with 9 decimals (nanomina).
    pub fn mina() -> Self {
        Currency {
            symbol: crate::MINA_SYMBOL.to_string(),
            decimals: crate::MINA_DECIMALS,
            metadata: None,
        }
    }

    /// The token id, or None for MINA.
    pub fn token_id(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.token_id.as_str())
    }
}
