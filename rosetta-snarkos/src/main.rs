use mentat::{cache::DefaultCacheInner, serve, server::Server, tokio};

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod macros;
mod node;
mod request;
mod responses;

use request::SnarkosJrpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(
        call_api::SnarkosCallApi::default(),
        construction_api::SnarkosConstructionApi::default(),
        data_api::SnarkosDataApi::default(),
        indexer_api::SnarkosIndexerApi::default(),
    );

    serve!(server, node::SnarkOSNode::default(), DefaultCacheInner)
}
