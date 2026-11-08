use serde::{Deserialize, Serialize};

/// The scheme of a signature. Mina signs with Schnorr over Poseidon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureType {
    Ecdsa,
    EcdsaRecovery,
    Ed25519,
    #[serde(rename = "schnorr_1")]
    Schnorr1,
    #[default]
    SchnorrPoseidon,
}
