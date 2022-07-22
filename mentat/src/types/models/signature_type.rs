//! The module defines the `SignatureType` model.

use std::fmt;

use super::*;

/// OperatorSignatureType is the type of a cryptographic signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct SignatureType(String);

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

    /// returns true if the `SignatureType` is a valid type
    pub fn valid(&self) -> bool {
        matches!(
            self.0.as_str(),
            Self::ECDSA
                | Self::ECDSA_RECOVERY
                | Self::ED25519
                | Self::SCHNORR_1
                | Self::SCHNORR_POSEIDON
        )
    }

    /// returns true if `SignatureType` contains an empty string
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for SignatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SignatureType {
    fn from(st: String) -> Self {
        Self(st)
    }
}

impl From<&str> for SignatureType {
    fn from(st: &str) -> Self {
        st.to_string().into()
    }
}
