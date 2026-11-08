use serde::{Deserialize, Serialize};

/// The curve of a public key. Mina keys are on Pallas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    Secp256k1,
    Secp256r1,
    Edwards25519,
    Tweedle,
    #[default]
    Pallas,
}
