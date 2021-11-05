use super::*;

/// The network_identifier specifies which network a particular object is associated with.
#[derive(Serialize, Deserialize)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    /// If a blockchain has a specific chain-id or network identifier, it should go in this field. It is up to the client to determine which network-specific identifier is mainnet or testnet.
    pub network: String,
    /// In blockchains with sharded state, the SubNetworkIdentifier is required to query some object on a specific shard. This identifier is optional for all non-sharded blockchains.
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}