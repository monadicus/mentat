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
struct SnarkosTypes;

impl ServerTypes for SnarkosTypes {
    type CallApi = call_api::SnarkosCallApi;
    type ConstructionApi = construction_api::SnarkosConstructionApi;
    type CustomConfig = NodeConfig;
    type DataApi = data_api::SnarkosDataApi;
    type IndexerApi = indexer_api::SnarkosIndexerApi;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::<SnarkosTypes>::builder()
        .call_api(call_api::SnarkosCallApi::default())
        .custom_configuration_from_arg()
        .construction_api(construction_api::SnarkosConstructionApi::default())
        .data_api(data_api::SnarkosDataApi::default())
        .indexer_api(indexer_api::SnarkosIndexerApi::default())
        .build();

    serve!(server, SnarkosTypes,)
}
