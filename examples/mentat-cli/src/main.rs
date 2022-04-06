use clap::Parser;
use mentat::{
    anyhow,
    client::Client,
    requests::{AccountBalanceRequest, AccountCoinsRequest, MetadataRequest},
    serde_json::json,
    tokio,
};

use crate::{account::AccountSubCommand, network::NetworkSubCommand};

mod account;
mod network;

#[derive(Parser)]
#[clap(version = "0.0.0", author = "Monadicus")]
struct Opts {
    #[clap(short, long, default_value = "http://127.0.0.1:8080")]
    endpoint: String,
    #[clap(short, long)]
    json: bool,
    #[clap(subcommand)]
    subcmd: MainSubCommand,
}

#[derive(Parser)]
enum MainSubCommand {
    Network(network::NetworkOpts),
    Account(account::AccountOpts),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let main_opts: Opts = Opts::parse();

    let client = Client::new(&main_opts.endpoint)?;

    // helper macro for automatically handling json output and errors
    macro_rules! display {
        // without an arrow, all inputs are outputted as json
        ($input:expr) => {
            match $input.await {
                Ok(res) => {
                    println!("{}", json!(res));
                }
                Err(e) => panic!("{}", e),
            }
        };
        // with an arrow, all outputs are json when the -j flag is passed
        ($input:expr, $res:ident => $body:expr) => {
            match $input.await {
                Ok($res) => {
                    if main_opts.json {
                        println!("{}", json!($res));
                    } else {
                        $body
                    }
                }
                Err(e) => panic!("{}", e),
            }
        };
    }

    match &main_opts.subcmd {
        MainSubCommand::Network(sub_opts) => match &sub_opts.subcmd {
            NetworkSubCommand::List => {
                display!(client.network_list(&MetadataRequest::default()), resp => {
                    network::list_table(resp.network_identifiers);
                })
            }
            NetworkSubCommand::Options(opts) => {
                let network =
                    network::first_network_or_null(&client, opts.net_id(), main_opts.json).await?;

                display!(client.network_options(&network.into()));
            }
            NetworkSubCommand::Status(opts) => {
                let network =
                    network::first_network_or_null(&client, opts.net_id(), main_opts.json).await?;

                display!(client.network_status(&network.into()), resp => {
                    network::status_table(resp);
                });
            }
        },
        MainSubCommand::Account(sub_opts) => match &sub_opts.subcmd {
            AccountSubCommand::Balance(opts) => {
                let network =
                    network::first_network_or_null(&client, sub_opts.net_id(), main_opts.json)
                        .await?;

                display!(client.account_balance(&AccountBalanceRequest {
                    network_identifier: network,
                    account_identifier: sub_opts.account_id(),
                    currencies: sub_opts.get_currencies(),
                    block_identifier: if opts.index.is_some() || opts.hash.is_some() {
                        Some(opts.clone().into())
                    } else {
                        None
                    },
                }));
            }
            AccountSubCommand::Coins => {
                let network =
                    network::first_network_or_null(&client, sub_opts.net_id(), main_opts.json)
                        .await?;

                display!(client.account_coins(&AccountCoinsRequest {
                    network_identifier: network,
                    account_identifier: sub_opts.account_id(),
                    currencies: sub_opts.get_currencies(),
                    ..Default::default()
                }));
            }
        },
    }

    Ok(())
}
