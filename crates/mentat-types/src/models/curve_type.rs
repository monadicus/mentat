//! The module defines the `CurveType` model.

use std::fmt;

use super::*;

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UncheckedCurveType(String);

impl UncheckedCurveType {
    /// <https://ed25519.cr.yp.to/ed25519-20110926.pdf>
    pub const EDWARDS25519: &'static str = "edwards25519";
    /// https://github.com/zcash/pasta
    pub const PALLAS: &'static str = "pallas";
    /// <https://secg.org/sec1-v2.pdf#subsubsection.2.3.3>
    pub const SECP256K1: &'static str = "secp256k1";
    /// <https://secg.org/sec1-v2.pdf#subsubsection.2.3.3>
    pub const SECP256R1: &'static str = "secp256r1";
    /// <https://github.com/CodaProtocol/coda/blob/develop/rfcs/0038-rosetta-construction-api.md#marshal-keys>
    pub const TWEEDLE: &'static str = "tweedle";

    /// returns true if the `CurveType` is a valid type
    pub fn valid(&self) -> bool {
        matches!(
            self.0.as_str(),
            Self::SECP256K1 | Self::SECP256R1 | Self::EDWARDS25519 | Self::TWEEDLE
        )
    }

    /// returns true if the underlying string is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for UncheckedCurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UncheckedCurveType {
    fn from(ct: String) -> Self {
        Self(ct)
    }
}

impl From<&str> for UncheckedCurveType {
    fn from(ct: &str) -> Self {
        ct.to_string().into()
    }
}

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    /// <https://ed25519.cr.yp.to/ed25519-20110926.pdf>
    #[default]
    Edwards25519,
    /// <https://secg.org/sec1-v2.pdf#subsubsection.2.3.3>
    Secp256k1,
    /// <https://secg.org/sec1-v2.pdf#subsubsection.2.3.3>
    Secp256r1,
    /// <https://github.com/CodaProtocol/coda/blob/develop/rfcs/0038-rosetta-construction-api.md#marshal-keys>
    Tweedle,
    /// https://github.com/zcash/pasta
    Pallas,
}

impl From<UncheckedCurveType> for CurveType {
    fn from(other: UncheckedCurveType) -> Self {
        match other.0.to_ascii_lowercase().as_ref() {
            UncheckedCurveType::EDWARDS25519 => Self::Edwards25519,
            UncheckedCurveType::SECP256K1 => Self::Secp256k1,
            UncheckedCurveType::SECP256R1 => Self::Secp256r1,
            UncheckedCurveType::TWEEDLE => Self::Tweedle,
            UncheckedCurveType::PALLAS => Self::Pallas,
            i => panic!("unsupported CurveType: {i}"),
        }
    }
}

impl From<CurveType> for UncheckedCurveType {
    fn from(other: CurveType) -> Self {
        match other {
            CurveType::Edwards25519 => Self::EDWARDS25519.into(),
            CurveType::Secp256k1 => Self::SECP256K1.into(),
            CurveType::Secp256r1 => Self::SECP256R1.into(),
            CurveType::Tweedle => Self::TWEEDLE.into(),
            CurveType::Pallas => Self::PALLAS.into(),
        }
    }
}

impl fmt::Display for CurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CurveType::Edwards25519 => write!(f, "edwards25519"),
            CurveType::Secp256k1 => write!(f, "secp256k1"),
            CurveType::Secp256r1 => write!(f, "secp256r1"),
            CurveType::Tweedle => write!(f, "tweedle"),
            CurveType::Pallas => write!(f, "pallas"),
        }
    }
}
