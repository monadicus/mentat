//! The module defines the `SignatureType` model.

use super::*;

/// OperatorSignatureType is the type of a cryptographic signature.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureType(pub String);

impl SignatureType {
    /// r (32-bytes) + s (32-bytes)
    pub const ECDSA: &'static str = "ecdsa";
    /// r (32-bytes) + s (32-bytes) + v (1-byte)
    pub const ECDSA_RECOVERY: &'static str = "ecdsa_recovery";
    /// R (32-bytes) + s (32-bytes)
    pub const ED25519: &'static str = "ed25519";
    /// r (32-bytes) + s (32-bytes)
    pub const SCHNORR_1: &'static str = "schnorr_1";
    /// r (32-bytes) + s (32-bytes) where s = Hash(1st pk + 2nd pk + r)
    pub const SCHNORR_POSEIDON: &'static str = "schnorr_poseidon";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::ECDSA
            | Self::ECDSA_RECOVERY
            | Self::ED25519
            | Self::SCHNORR_1
            | Self::SCHNORR_POSEIDON => true,
            _ => false,
        }
    }
}

impl Default for SignatureType {
    fn default() -> Self {
        Self(Self::ECDSA.to_string())
    }
}
