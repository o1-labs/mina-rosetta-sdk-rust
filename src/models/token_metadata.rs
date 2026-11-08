use serde::{Deserialize, Serialize};

/// The metadata Mina puts on an account or a currency of a token other
/// than MINA: the token's id.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub token_id: String,
}
