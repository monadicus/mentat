//! The module defines the `NetworkIdentifier`.

#[cfg(feature = "server")]
use axum::http::Extensions;

use super::*;
use crate::types::{NetworkRequest, Result, Sortable};
#[cfg(feature = "server")]
use crate::{
    conf::{Configuration, Network, NodeConf},
    server::ServerType,
};

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

impl From<NetworkIdentifier> for NetworkRequest {
    fn from(net: NetworkIdentifier) -> Self {
        Self {
            network_identifier: Some(net),
            ..Default::default()
        }
    }
}

impl From<Option<NetworkIdentifier>> for NetworkRequest {
    fn from(network_identifier: Option<NetworkIdentifier>) -> Self {
        Self {
            network_identifier,
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

#[cfg(feature = "server")]
impl NetworkIdentifier {
    /// A function to check if the server Blockchain specified matches the user
    /// request specified blockchain.
    pub fn check<Types: ServerType>(extensions: &Extensions, json: &Value) -> Result<()> {
        let config = extensions
            .get::<Configuration<Types::CustomConfig>>()
            .unwrap();
        if let Some(net_id) = json.get("network_identifier") {
            let network_identifier = serde_json::from_value::<Self>(net_id.clone())?;
            if network_identifier.blockchain.to_uppercase()
                != Types::CustomConfig::BLOCKCHAIN.to_uppercase()
            {
                return Err(format!(
                    "invalid blockchain ID: found `{}`, expected `{}`",
                    network_identifier.blockchain.to_uppercase(),
                    Types::CustomConfig::BLOCKCHAIN.to_uppercase()
                )
                .into());
            } else if Network::from(network_identifier.network.to_uppercase()) != config.network {
                return Err(format!(
                    "invalid network ID: found `{}`, expected `{}`",
                    network_identifier.network.to_uppercase(),
                    config.network
                )
                .into());
            }
        }
        Ok(())
    }
}
