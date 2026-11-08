use super::SubNetworkIdentifier;
use serde::{Deserialize, Serialize};

/// The network a request is for. On Mina `blockchain` is `mina` and
/// `network` is the daemon's network id: `mainnet`, `devnet`, `testnet`
/// or the name of a local network.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    pub network: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

impl NetworkIdentifier {
    /// The Mina network called `network`.
    pub fn mina(network: impl Into<String>) -> Self {
        NetworkIdentifier {
            blockchain: crate::BLOCKCHAIN.to_string(),
            network: network.into(),
            sub_network_identifier: None,
        }
    }
}
