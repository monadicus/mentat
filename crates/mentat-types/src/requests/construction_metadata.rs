//! The module defines the `ConstructionMetadataRequest` request.

use super::*;

/// A [`ConstructionMetadataRequest`] is utilized to get information required to
/// construct a transaction. The `Options` object used to specify which metadata
/// to return is left purposely unstructured to allow flexibility for
/// implementers. `Options` is not required in the case that there is
/// network-wide metadata of interest. Optionally, the request can also include
/// an array of [`PublicKey`]s associated with the [`Account
/// #[serde(default)]Identifier`]s
/// returned in [`crate::responses::ConstructionPreprocessResponse`].
#[derive(Clone, Debug, Deserialize, Serialize, Default, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedConstructionMetadataRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// Some blockchains require different metadata for different types of
    /// transaction construction (ex: delegation versus a transfer). Instead of
    /// requiring a blockchain node to return all possible types of metadata for
    /// construction (which may require multiple node fetches), the client can
    /// populate an options object to limit the metadata returned to only the
    /// subset required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub options: Option<Value>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub public_keys: Vec<Option<UncheckedPublicKey>>,
}
