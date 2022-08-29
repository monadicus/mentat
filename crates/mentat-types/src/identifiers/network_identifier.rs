//! The module defines the `NetworkIdentifier`.

use super::*;

/// The [`NetworkIdentifier`] specifies which network a particular object is
/// associated with.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkIdentifier {
    /// The name of the blockchain.
    #[serde(
        serialize_with = "string_to_uppercase",
        deserialize_with = "string_as_uppercase"
    )]
    pub blockchain: String,
    /// If a blockchain has a specific chain-id or network identifier, it should
    /// go in this field. It is up to the client to determine which
    /// network-specific identifier is mainnet or testnet.
    #[serde(
        serialize_with = "string_to_uppercase",
        deserialize_with = "string_as_uppercase"
    )]
    pub network: String,
    /// In blockchains with sharded state, the `SubNetworkIdentifier` is
    /// required to query some object on a specific shard. This identifier
    /// is optional for all non-sharded blockchains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

impl From<(&str, &str)> for NetworkIdentifier {
    fn from((blockchain, network): (&str, &str)) -> Self {
        Self {
            blockchain: blockchain.to_uppercase(),
            network: network.to_uppercase(),
            ..Default::default()
        }
    }
}

impl From<(&str, &str, &str)> for NetworkIdentifier {
    fn from((blockchain, network, subnet): (&str, &str, &str)) -> Self {
        Self {
            blockchain: blockchain.to_uppercase(),
            network: network.to_uppercase(),
            sub_network_identifier: Some(subnet.into()),
        }
    }
}

impl From<(&str, &str, Option<&str>)> for NetworkIdentifier {
    fn from((blockchain, network, subnet): (&str, &str, Option<&str>)) -> Self {
        Self {
            blockchain: blockchain.to_uppercase(),
            network: network.to_uppercase(),
            sub_network_identifier: subnet.map(|s| s.into()),
        }
    }
}

impl From<Option<NetworkIdentifier>> for UncheckedNetworkRequest {
    fn from(net: Option<NetworkIdentifier>) -> Self {
        Self {
            network_identifier: net,
            ..Default::default()
        }
    }
}

impl Sortable for NetworkIdentifier {
    fn sort(&self) -> Self {
        Self {
            blockchain: self.blockchain.to_uppercase(),
            network: self.network.to_uppercase(),
            sub_network_identifier: self.sub_network_identifier.clone().map(|sni| sni.sort()),
        }
    }
}
