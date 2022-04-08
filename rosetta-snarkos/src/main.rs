mod additional_api;
mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod macros;
mod node;
mod request;
mod responses;

use mentat::{
    cache::DefaultCacheInner,
    server::{Server, ServerType},
};

#[derive(Clone)]
struct MentatSnarkos;

impl ServerType for MentatSnarkos {
    type AdditionalApi = additional_api::SnarkosAdditionalApi;
    type CallApi = call_api::SnarkosCallApi;
    type ConstructionApi = construction_api::SnarkosConstructionApi;
    type CustomConfig = node::NodeConfig;
    type DataApi = data_api::SnarkosDataApi;
    type IndexerApi = indexer_api::SnarkosIndexerApi;
}

#[mentat::main(DefaultCacheInner)]
async fn main() -> Server<MentatSnarkos> {
    println!("hello rosetta!");
    Server::default()
}
