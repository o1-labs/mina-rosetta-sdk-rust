use super::CurveType;
use serde::{Deserialize, Serialize};

/// A public key as hex bytes: on Mina a Pallas point, as `mina-signer`'s
/// `publicKeyToRaw` writes it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PublicKey {
    pub hex_bytes: String,
    pub curve_type: CurveType,
}

impl PublicKey {
    /// A Mina (Pallas) public key.
    pub fn pallas(hex_bytes: impl Into<String>) -> Self {
        PublicKey {
            hex_bytes: hex_bytes.into(),
            curve_type: CurveType::Pallas,
        }
    }
}
