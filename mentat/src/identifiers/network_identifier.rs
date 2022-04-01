use axum::http::Extensions;

use super::*;
use crate::{
    conf::{Configuration, Network},
    errors::{MentatError, Result},
    server::ServerType,
};

/// The network_identifier specifies which network a particular object is
/// associated with.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    /// If a blockchain has a specific chain-id or network identifier, it should
    /// go in this field. It is up to the client to determine which
    /// network-specific identifier is mainnet or testnet.
    pub network: String,
    /// In blockchains with sharded state, the SubNetworkIdentifier is required
    /// to query some object on a specific shard. This identifier is optional
    /// for all non-sharded blockchains.
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

impl NetworkIdentifier {
    pub async fn check<Types: ServerType>(extensions: &Extensions, json: &Value) -> Result<()> {
        let config = extensions
            .get::<Configuration<Types::CustomConfig>>()
            .unwrap();
        if let Some(net_id) = json.get("network_identifier") {
            let network_identifier = serde_json::from_value::<Self>(net_id.clone())?;
            if network_identifier.blockchain.to_uppercase() != config.blockchain.to_uppercase() {
                return Err(MentatError::from(format!(
                    "invalid blockchain ID: found `{}`, expected `{}`",
                    network_identifier.blockchain.to_uppercase(),
                    config.blockchain.to_uppercase()
                )));
            } else if Network::from(network_identifier.network.to_uppercase()) != config.network {
                return Err(MentatError::from(format!(
                    "invalid network ID: found `{}`, expected `{}`",
                    network_identifier.network.to_uppercase(),
                    config.network
                )));
            }
        }
        Ok(())
    }
}
