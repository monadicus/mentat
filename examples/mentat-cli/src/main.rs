use clap::Parser;
use mentat::{anyhow, client::Client, requests::MetadataRequest, serde_json::json, tokio};

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let main_opts: Opts = Opts::parse();

    let mut client = Client::new(&main_opts.endpoint)?;

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

    match main_opts.subcmd {
        MainSubCommand::Network(sub_opts) => match sub_opts.subcmd {
            network::NetworkSubCommand::List(_opts) => {
                display!(client.network_list(&MetadataRequest::default()), resp => {
                    network::list_table(resp.network_identifiers);
                })
            }
            network::NetworkSubCommand::Options(opts) => {
                display!(client.network_options(&opts.into()))
            }
            network::NetworkSubCommand::Status(opts) => {
                display!(client.network_status(&opts.into()), resp => {
                    network::status_table(resp);
                })
            }
        },
    }

    Ok(())
}
