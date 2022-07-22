//! The module defines the `CurveType` model.

use std::fmt;

use super::*;

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CurveType(String);

impl CurveType {
    /// <https://ed25519.cr.yp.to/ed25519-20110926.pdf>
    pub const EDWARDS25519: &'static str = "edwards25519";
    /// <https://secg.org/sec1-v2.pdf#subsubsection.2.3.3>
    pub const SECP256K1: &'static str = "secp256k1";
    /// <https://secg.org/sec1-v2.pdf#subsubsection.2.3.3>
    pub const SECP256R1: &'static str = "secp256r1";
    /// <https://github.com/CodaProtocol/coda/blob/develop/rfcs/0038-rosetta-construction-api.md#marshal-keys>
    pub const TWEEDLE: &'static str = "tweedle";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::SECP256K1 | Self::SECP256R1 | Self::EDWARDS25519 | Self::TWEEDLE => true,
            _ => false,
        }
    }
}

impl fmt::Display for CurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CurveType {
    fn from(ct: String) -> Self {
        Self(ct)
    }
}

impl From<&str> for CurveType {
    fn from(ct: &str) -> Self {
        ct.to_string().into()
    }
}
