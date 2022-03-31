use mentat::{
    cache::DefaultCacheInner,
    serve,
    server::{Server, ServerBuilder},
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

impl ServerBuilder for MentatSnarkos {
    type CallApi = call_api::SnarkosCallApi;
    type ConstructionApi = construction_api::SnarkosConstructionApi;
    type CustomConfig = NodeConfig;
    type DataApi = data_api::SnarkosDataApi;
    type IndexerApi = indexer_api::SnarkosIndexerApi;
}

#[mentat::main(MentatSnarkos, DefaultCacheInner)]
async fn main() {
    println!("hello rosetta!");
}
