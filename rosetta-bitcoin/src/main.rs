use std::path::PathBuf;

use mentat::{serve, server::Server, tokio};

use crate::node::NodeConfig;

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod macros;
mod node;
mod request;
mod responses;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("conf.toml");

    let server = Server::<NodeConfig>::new(
        call_api::BitcoinCallApi::default(),
        construction_api::BitcoinConstructionApi::default(),
        &path,
        data_api::BitcoinDataApi::default(),
        indexer_api::BitcoinIndexerApi::default(),
    );

    serve!(server, NodeConfig,)
}
