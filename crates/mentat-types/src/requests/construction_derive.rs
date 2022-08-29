//! The module defines the ConstructionDeriveRequest request.

use indexmap::IndexMap;

use super::*;

/// [`ConstructionDeriveRequest`] is passed to the `/construction/derive`
/// endpoint. Network is provided in the request because some blockchains have
/// different address formats for different networks. `Metadata` is provided in
/// the request because some blockchains allow for multiple address types (i.e.
/// different address for validators vs normal accounts).
#[derive(Debug, Deserialize, Serialize, Default, Unchecked)]
#[serde(default)]
pub struct UncheckedConstructionDeriveRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// [`PublicKey`] contains a public key byte array for a particular
    /// [`CurveType`] encoded in hex. Note that there is no `PrivateKey`
    /// struct as this is NEVER the concern of an implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<UncheckedPublicKey>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: IndexMap<String, Value>,
}
