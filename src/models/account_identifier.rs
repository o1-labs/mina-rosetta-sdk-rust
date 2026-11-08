use super::{SubAccountIdentifier, TokenMetadata};
use serde::{Deserialize, Serialize};

/// An account: a public key and, for a token other than MINA, the token id.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AccountIdentifier {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<SubAccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TokenMetadata>,
}

impl AccountIdentifier {
    /// The MINA account of `address`.
    pub fn new(address: impl Into<String>) -> Self {
        AccountIdentifier {
            address: address.into(),
            ..Default::default()
        }
    }

    /// The account of `address` for the token `token_id`.
    pub fn with_token(address: impl Into<String>, token_id: impl Into<String>) -> Self {
        AccountIdentifier {
            address: address.into(),
            sub_account: None,
            metadata: Some(TokenMetadata {
                token_id: token_id.into(),
            }),
        }
    }

    /// The token id, or None for MINA.
    pub fn token_id(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.token_id.as_str())
    }
}

impl From<&str> for AccountIdentifier {
    fn from(address: &str) -> Self {
        AccountIdentifier::new(address)
    }
}

impl From<String> for AccountIdentifier {
    fn from(address: String) -> Self {
        AccountIdentifier::new(address)
    }
}
