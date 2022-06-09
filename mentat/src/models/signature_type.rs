//! The module defines the `SignatureType` model.

use super::*;

/// OperatorSignatureType is the type of a cryptographic signature.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SignatureType {
    /// r (32-bytes) + s (32-bytes)
    #[serde(rename = "ecdsa")]
    Ecdsa,
    /// r (32-bytes) + s (32-bytes) + v (1-byte)
    #[serde(rename = "ecdsa_recovery")]
    EcdsaRecovery,
    /// R (32-bytes) + s (32-bytes)
    #[serde(rename = "ed25519")]
    Ed25519,
    /// r (32-bytes) + s (32-bytes)
    #[serde(rename = "schnorr_1")]
    Schnorr1,
    /// r (32-bytes) + s (32-bytes) where s = Hash(1st pk + 2nd pk + r)
    #[serde(rename = "schnorr_poseidon")]
    SchnorrPoseidon,
}
