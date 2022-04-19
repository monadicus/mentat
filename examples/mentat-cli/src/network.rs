// use clap::Parser;
// use mentat::{
//     anyhow::Result,
//     client::Client,
//     identifiers::NetworkIdentifier,
//     requests::MetadataRequest,
// };

// #[derive(Parser)]
// pub(crate) struct NetworkOpts {
//     #[clap(subcommand)]
//     pub(crate) subcmd: NetworkSubCommand,
// }

// #[derive(Parser)]
// pub(crate) enum NetworkSubCommand {
//     List,
//     Status,
//     Options,
// }

// /// Get the first network from the networks list or use the given network
// /// identifier
// pub(crate) async fn get_network(
//     client: &Client,
//     net_ident: NetworkIdentifier,
// ) -> Result<Option<NetworkIdentifier>> {
//     Ok(Some(
//         if net_ident.blockchain.is_empty() && net_ident.network.is_empty() {
//             // get the first network
//             if let Some(net) = client
//                 .network_list(&MetadataRequest::default())
//                 .await?
//                 .network_identifiers
//                 .first()
//             {
//                 net.clone()
//             } else {
//                 return Ok(None);
//             }
//         } else {
//             net_ident
//         },
//     ))
// }

// /// Gets the first network in the networks list, prints "null" or "network not
// /// found" when the network doesn't exist
// pub(crate) async fn first_network_or_null(
//     client: &Client,
//     net_ident: NetworkIdentifier,
// ) -> Result<NetworkIdentifier> {
//     Ok(get_network(client, net_ident).await?.unwrap_or_else(|| {
//         println!("null");
//         std::process::exit(0)
//     }))
// }
