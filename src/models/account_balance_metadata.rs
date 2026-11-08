use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// What Mina adds to `/account/balance`: the account's nonce, and whether
/// the balance came from the archive rather than the daemon.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AccountBalanceMetadata {
    #[serde(default)]
    pub created_via_historical_lookup: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u32,
}
