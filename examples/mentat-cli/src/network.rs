use clap::Parser;
use mentat::{
    anyhow::Result,
    client::Client,
    identifiers::{NetworkIdentifier, SubNetworkIdentifier},
    requests::MetadataRequest,
};

#[derive(Parser)]
pub(crate) struct NetworkOpts {
    #[clap(subcommand)]
    pub(crate) subcmd: NetworkSubCommand,
}

#[derive(Parser)]
pub(crate) enum NetworkSubCommand {
    List,
    Status(NetworkStatusOpts),
    Options(NetworkStatusOpts),
}

#[derive(Parser)]
pub(crate) struct NetworkStatusOpts {
    #[clap(short, long, default_value = "")]
    pub(crate) blockchain: String,
    #[clap(short, long, default_value = "")]
    pub(crate) network: String,
    #[clap(short, long, default_value = "")]
    pub(crate) subnetwork: String,
}

impl NetworkStatusOpts {
    pub(crate) fn net_id(&self) -> NetworkIdentifier {
        NetworkIdentifier {
            blockchain: self.blockchain.clone(),
            network: self.network.clone(),
            sub_network_identifier: if self.subnetwork.len() > 0 {
                Some(SubNetworkIdentifier {
                    network: self.subnetwork.clone(),
                    ..Default::default()
                })
            } else {
                None
            },
        }
    }
}

/// Get the first network from the networks list or use the given network
/// identifier
pub(crate) async fn get_network(
    client: &Client,
    net_ident: NetworkIdentifier,
) -> Result<Option<NetworkIdentifier>> {
    Ok(Some(
        if net_ident.blockchain.len() == 0 && net_ident.network.len() == 0 {
            // get the first network
            if let Some(net) = client
                .network_list(&MetadataRequest::default())
                .await?
                .network_identifiers
                .first()
            {
                net.clone()
            } else {
                return Ok(None);
            }
        } else {
            net_ident
        },
    ))
}

/// Gets the first network in the networks list, prints "null" or "network not
/// found" when the network doesn't exist
pub(crate) async fn first_network_or_null(
    client: &Client,
    net_ident: NetworkIdentifier,
) -> Result<NetworkIdentifier> {
    Ok(get_network(&client, net_ident).await?.unwrap_or_else(|| {
        println!("null");
        std::process::exit(0)
    }))
}
