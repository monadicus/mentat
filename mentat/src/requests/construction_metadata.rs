//! The module defines the `ConstructionMetadataRequest` request.

use super::*;

/// A `ConstructionMetadataRequest` is utilized to get information required to
/// construct a transaction. The `Options` object used to specify which metadata
/// to return is left purposely unstructured to allow flexibility for
/// implementers. `Options` is not required in the case that there is
/// network-wide metadata of interest. Optionally, the request can also include
/// an array of [`PublicKey`]s associated with the [`AccountIdentifier`]s
/// returned in [`crate::responses::ConstructionPreprocessResponse`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionMetadataRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Some blockchains require different metadata for different types of
    /// transaction construction (ex: delegation versus a transfer). Instead of
    /// requiring a blockchain node to return all possible types of metadata for
    /// construction (which may require multiple node fetches), the client can
    /// populate an options object to limit the metadata returned to only the
    /// subset required.
    pub options: Option<Value>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub public_keys: Option<Vec<PublicKey>>,
}
