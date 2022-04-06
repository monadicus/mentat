//! The module defines the CurveType model.

use super::*;

/// CurveType is the type of cryptographic curve associated with a PublicKey.
#[derive(Debug, Deserialize, Serialize)]
pub enum CurveType {
    /// https://secg.org/sec1-v2.pdf#subsubsection.2.3.3
    #[serde(rename = "secp256k1")]
    Secp256k1,
    /// https://secg.org/sec1-v2.pdf#subsubsection.2.3.3
    #[serde(rename = "secp256r1")]
    Secp256r1,
    /// https://ed25519.cr.yp.to/ed25519-20110926.pdf
    #[serde(rename = "edwards25519")]
    Edwards25519,
    /// https://github.com/CodaProtocol/coda/blob/develop/rfcs/0038-rosetta-construction-api.md#marshal-keys
    #[serde(rename = "tweedle")]
    Tweedle,
}
