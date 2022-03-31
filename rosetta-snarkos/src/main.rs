use mentat::{
    serve,
    server::{Server, ServerTypes},
    tokio,
};

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod macros;
mod node;
mod request;
mod responses;

use request::SnarkosJrpc;

use crate::node::NodeConfig;

#[derive(Clone)]
struct MentatSnarkos;

impl ServerTypes for MentatSnarkos {
    type CallApi = call_api::SnarkosCallApi;
    type ConstructionApi = construction_api::SnarkosConstructionApi;
    type CustomConfig = NodeConfig;
    type DataApi = data_api::SnarkosDataApi;
    type IndexerApi = indexer_api::SnarkosIndexerApi;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    serve!(MentatSnarkos)
}
