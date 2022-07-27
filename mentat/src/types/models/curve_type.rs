//! The module defines the `CurveType` model.

use std::fmt;

use super::*;

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NullableCurveType(String);

impl NullableCurveType {
    /// <https://ed25519.cr.yp.to/ed25519-20110926.pdf>
    pub const EDWARDS25519: &'static str = "edwards25519";
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
}

impl fmt::Display for NullableCurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NullableCurveType {
    fn from(ct: String) -> Self {
        Self(ct)
    }
}

impl From<&str> for NullableCurveType {
    fn from(ct: &str) -> Self {
        ct.to_string().into()
    }
}

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Clone, Debug, Default)]
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
}

impl From<NullableCurveType> for CurveType {
    fn from(other: NullableCurveType) -> Self {
        match other.0.as_ref() {
            NullableCurveType::EDWARDS25519 => Self::Edwards25519,
            NullableCurveType::SECP256K1 => Self::Secp256k1,
            NullableCurveType::SECP256R1 => Self::Secp256r1,
            NullableCurveType::TWEEDLE => Self::Tweedle,
            i => panic!("unsupported CurveType: {i}"),
        }
    }
}

impl From<CurveType> for NullableCurveType {
    fn from(other: CurveType) -> Self {
        match other {
            CurveType::Edwards25519 => Self::EDWARDS25519.into(),
            CurveType::Secp256k1 => Self::SECP256K1.into(),
            CurveType::Secp256r1 => Self::SECP256R1.into(),
            CurveType::Tweedle => Self::TWEEDLE.into(),
        }
    }
}
