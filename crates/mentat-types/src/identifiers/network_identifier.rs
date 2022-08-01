//! The module defines the `NetworkIdentifier`.

use super::*;

/// The [`NetworkIdentifier`] specifies which network a particular object is
/// associated with.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkIdentifier {
    /// The name of the blockchain.
    pub blockchain: String,
    /// If a blockchain has a specific chain-id or network identifier, it should
    /// go in this field. It is up to the client to determine which
    /// network-specific identifier is mainnet or testnet.
    pub network: String,
    /// In blockchains with sharded state, the `SubNetworkIdentifier` is
    /// required to query some object on a specific shard. This identifier
    /// is optional for all non-sharded blockchains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

impl From<(String, String)> for NetworkIdentifier {
    fn from((blockchain, network): (String, String)) -> Self {
        Self {
            blockchain,
            network,
            ..Default::default()
        }
    }
}

impl From<(String, String, String)> for NetworkIdentifier {
    fn from((blockchain, network, subnet): (String, String, String)) -> Self {
        Self {
            blockchain,
            network,
            sub_network_identifier: Some(subnet.into()),
        }
    }
}

impl From<(String, String, Option<String>)> for NetworkIdentifier {
    fn from((blockchain, network, subnet): (String, String, Option<String>)) -> Self {
        Self {
            blockchain,
            network,
            sub_network_identifier: subnet.map(|s| s.into()),
        }
    }
}

impl From<Option<NetworkIdentifier>> for NullableNetworkRequest {
    fn from(net: Option<NetworkIdentifier>) -> Self {
        Self {
            network_identifier: net,
            ..Default::default()
        }
    }
}

impl Sortable for NetworkIdentifier {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.sub_network_identifier = new.sub_network_identifier.map(|sni| sni.sort());
        new
    }
}
