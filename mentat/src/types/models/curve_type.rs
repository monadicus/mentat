//! The module defines the `CurveType` model.

use super::*;

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CurveType(pub String);

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

impl Default for CurveType {
    fn default() -> Self {
        Self(Self::SECP256K1.to_string())
    }
}
