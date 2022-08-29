//! The module defines the `SignatureType` model.

use std::fmt;

use super::*;

/// OperatorSignatureType is the type of a cryptographic signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct UncheckedSignatureType(String);

impl UncheckedSignatureType {
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

impl fmt::Display for UncheckedSignatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UncheckedSignatureType {
    fn from(st: String) -> Self {
        Self(st)
    }
}

impl From<&str> for UncheckedSignatureType {
    fn from(st: &str) -> Self {
        st.to_string().into()
    }
}

/// OperatorSignatureType is the type of a cryptographic signature.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignatureType {
    /// r (32-bytes) + s (32-bytes)
    #[default]
    Ecdsa,
    /// r (32-bytes) + s (32-bytes) + v (1-byte)
    EcdsaRecovery,
    /// R (32-bytes) + s (32-bytes)
    Ed25519,
    /// r (32-bytes) + s (32-bytes)
    #[serde(rename = "schnorr_1")]
    Schnorr1,
    /// r (32-bytes) + s (32-bytes) where s = Hash(1st pk + 2nd pk + r)
    EchnorrPoseidon,
}

impl From<UncheckedSignatureType> for SignatureType {
    fn from(other: UncheckedSignatureType) -> Self {
        match other.0.as_ref() {
            UncheckedSignatureType::ECDSA => Self::Ecdsa,
            UncheckedSignatureType::ECDSA_RECOVERY => Self::EcdsaRecovery,
            UncheckedSignatureType::ED25519 => Self::Ed25519,
            UncheckedSignatureType::SCHNORR_1 => Self::Schnorr1,
            UncheckedSignatureType::SCHNORR_POSEIDON => Self::EchnorrPoseidon,
            i => panic!("unsupported ExemptionType: {i}"),
        }
    }
}

impl From<SignatureType> for UncheckedSignatureType {
    fn from(other: SignatureType) -> Self {
        match other {
            SignatureType::Ecdsa => Self::ECDSA.into(),
            SignatureType::EcdsaRecovery => Self::ECDSA_RECOVERY.into(),
            SignatureType::Ed25519 => Self::ED25519.into(),
            SignatureType::Schnorr1 => Self::SCHNORR_1.into(),
            SignatureType::EchnorrPoseidon => Self::SCHNORR_POSEIDON.into(),
        }
    }
}

impl fmt::Display for SignatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureType::Ecdsa => write!(f, "ecdsa"),
            SignatureType::EcdsaRecovery => write!(f, "ecdsa_recovery"),
            SignatureType::Ed25519 => write!(f, "ed25519"),
            SignatureType::Schnorr1 => write!(f, "schnorr_1"),
            SignatureType::EchnorrPoseidon => write!(f, "schnorr_poseidon"),
        }
    }
}
