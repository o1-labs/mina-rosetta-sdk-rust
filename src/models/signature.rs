use super::{PublicKey, SignatureType, SigningPayload};
use serde::{Deserialize, Serialize};

/// A signature of a signing payload, hex encoded.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Signature {
    pub signing_payload: SigningPayload,
    pub public_key: PublicKey,
    pub signature_type: SignatureType,
    pub hex_bytes: String,
}
