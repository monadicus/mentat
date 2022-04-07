//! The module defines the ConstructionDeriveRequest request.

use indexmap::IndexMap;

use super::*;

/// `ConstructionDeriveRequest` is passed to the `/construction/derive`
/// endpoint. Network is provided in the request because some blockchains have
/// different address formats for different networks. `Metadata` is provided in
/// the request because some blockchains allow for multiple address types (i.e.
/// different address for validators vs normal accounts).
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstructionDeriveRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// `PublicKey` contains a public key byte array for a particular
    /// [`CurveType`] encoded in hex. Note that there is no [`PrivateKey`]
    /// struct as this is NEVER the concern of an implementation.
    pub public_key: PublicKey,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
