use super::*;

/// A BlockRequest is utilized to make a block request on the /block endpoint.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockRequest {
    /// The network_identifier specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// When fetching data by BlockIdentifier, it may be possible to only
    /// specify the index or hash. If neither property is specified, it is
    /// assumed that the client is making a request at the current block.
    pub block_identifier: PartialBlockIdentifier,
}

impl From<(NetworkIdentifier, PartialBlockIdentifier)> for BlockRequest {
    fn from(
        (network_identifier, block_identifier): (NetworkIdentifier, PartialBlockIdentifier),
    ) -> Self {
        Self {
            network_identifier,
            block_identifier,
        }
    }
}

impl From<(PartialBlockIdentifier, NetworkIdentifier)> for BlockRequest {
    fn from(
        (block_identifier, network_identifier): (PartialBlockIdentifier, NetworkIdentifier),
    ) -> Self {
        Self {
            network_identifier,
            block_identifier,
        }
    }
}
