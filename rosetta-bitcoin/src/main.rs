use std::path::PathBuf;

use mentat::{
    serve,
    server::{DummyNode, Server},
    tokio,
};

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("conf.toml");

    let server = Server::<()>::new(
        call_api::BitcoinCallApi::default(),
        construction_api::BitcoinConstructionApi::default(),
        &path,
        data_api::BitcoinDataApi::default(),
        indexer_api::BitcoinIndexerApi::default(),
    );

    serve!(server, DummyNode::default(), (),)
}
