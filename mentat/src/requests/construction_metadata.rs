use super::*;

/// A ConstructionMetadataRequest is utilized to get information required to
/// construct a transaction. The Options object used to specify which metadata
/// to return is left purposely unstructured to allow flexibility for
/// implementers. Options is not required in the case that there is network-wide
/// metadata of interest. Optionally, the request can also include an array of
/// PublicKeys associated with the AccountIdentifiers returned in
/// ConstructionPreprocessResponse.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionMetadataRequest {
    /// The network_identifier specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Some blockchains require different metadata for different types of
    /// transaction construction (ex: delegation versus a transfer). Instead of
    /// requiring a blockchain node to return all possible types of metadata for
    /// construction (which may require multiple node fetches), the client can
    /// populate an options object to limit the metadata returned to only the
    /// subset required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<PublicKey>>,
}
