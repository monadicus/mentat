use clap::Parser;
use mentat::{
    anyhow,
    client::Client,
    identifiers::{
        BlockIdentifier,
        NetworkIdentifier,
        PartialBlockIdentifier,
        SubNetworkIdentifier,
        TransactionIdentifier,
    },
    requests::{
        AccountBalanceRequest,
        AccountCoinsRequest,
        BlockRequest,
        BlockTransactionRequest,
        MempoolTransactionRequest,
        MetadataRequest,
    },
    serde_json::json,
    tokio,
};

use crate::{account::AccountSubCommand, network::NetworkSubCommand};

mod account;
mod block;
mod mempool;
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

    #[clap(short, long, default_value = "")]
    pub(crate) blockchain: String,
    #[clap(short, long, default_value = "")]
    pub(crate) network: String,
    #[clap(short, long, default_value = "")]
    pub(crate) subnetwork: String,

    #[clap(long)]
    pub(crate) index: Option<u64>,
    #[clap(long)]
    pub(crate) hash: Option<String>,
}

impl Opts {
    /// Get a network identifier from flags
    fn net_id(&self) -> NetworkIdentifier {
        NetworkIdentifier {
            blockchain: self.blockchain.clone(),
            network: self.network.clone(),
            sub_network_identifier: if !self.subnetwork.is_empty() {
                Some(SubNetworkIdentifier {
                    network: self.subnetwork.clone(),
                    ..Default::default()
                })
            } else {
                None
            },
        }
    }

    /// Get a partial block id from flags
    fn partial_block_id(&self) -> Option<PartialBlockIdentifier> {
        match (&self.hash, &self.index) {
            (None, None) => None,
            _ => Some(PartialBlockIdentifier {
                hash: self.hash.clone(),
                index: self.index,
            }),
        }
    }

    /// Get a block identifier from flags
    fn block_id(&self) -> Option<BlockIdentifier> {
        match (&self.hash, &self.index) {
            (Some(hash), Some(index)) => Some(BlockIdentifier {
                hash: hash.clone(),
                index: *index,
            }),
            _ => None,
        }
    }
}

#[derive(Parser)]
enum MainSubCommand {
    Network(network::NetworkOpts),
    Account(account::AccountOpts),
    Block(block::BlockOpts),
    Mempool(mempool::MempoolOpts),
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
    }

    // Use the network from flags or use the first network in the list
    let get_first_network = || network::first_network_or_null(&client, main_opts.net_id());

    match &main_opts.subcmd {
        MainSubCommand::Network(sub_opts) => match &sub_opts.subcmd {
            NetworkSubCommand::List => {
                display!(client.network_list(&MetadataRequest::default()))
            }
            NetworkSubCommand::Options => {
                let network = get_first_network().await?;
                display!(client.network_options(&network.into()));
            }
            NetworkSubCommand::Status => {
                let network = get_first_network().await?;
                display!(client.network_status(&network.into()));
            }
        },
        MainSubCommand::Account(sub_opts) => match &sub_opts.subcmd {
            AccountSubCommand::Balance(_opts) => {
                let network = get_first_network().await?;
                display!(client.account_balance(&AccountBalanceRequest {
                    network_identifier: network,
                    account_identifier: sub_opts.account_id(),
                    currencies: sub_opts.get_currencies(),
                    block_identifier: main_opts.partial_block_id(),
                }));
            }
            AccountSubCommand::Coins(_opts) => {
                let network = get_first_network().await?;
                display!(client.account_coins(&AccountCoinsRequest {
                    network_identifier: network,
                    account_identifier: sub_opts.account_id(),
                    currencies: sub_opts.get_currencies(),
                    ..Default::default()
                }));
            }
        },
        MainSubCommand::Block(opts) => {
            let network = get_first_network().await?;
            // handle --transaction flag
            if let Some(transaction) = &opts.transaction {
                let block = main_opts
                    .block_id()
                    .expect("Expected a block identifier (--hash, --index)");
                display!(client.block_transaction(&BlockTransactionRequest {
                    network_identifier: network,
                    block_identifier: block,
                    transaction_identifier: TransactionIdentifier {
                        hash: transaction.clone()
                    },
                }));
            } else {
                // find a specific block
                let block = main_opts
                    .partial_block_id()
                    .expect("Expected a partial block identifier (--hash, --index)");
                display!(client.block(&BlockRequest {
                    network_identifier: network,
                    block_identifier: block,
                }));
            }
        }
        MainSubCommand::Mempool(opts) => {
            let network = get_first_network().await?;
            // handle --transaction flag
            if let Some(transaction) = &opts.transaction {
                display!(client.mempool_transaction(&MempoolTransactionRequest {
                    network_identifier: network,
                    transaction_identifier: TransactionIdentifier {
                        hash: transaction.clone()
                    },
                }));
            } else {
                display!(client.mempool(&network.into()));
            }
        }
    }

    Ok(())
}
