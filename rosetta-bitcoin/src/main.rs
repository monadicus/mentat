use mentat::{
    serve,
    server::{Server, ServerTypes},
    tokio,
};

use crate::node::NodeConfig;

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod macros;
mod node;
mod request;
mod responses;

#[derive(Clone)]
struct BitcoinTypes;

impl ServerTypes for BitcoinTypes {
    type CallApi = call_api::BitcoinCallApi;
    type ConstructionApi = construction_api::BitcoinConstructionApi;
    type CustomConfig = NodeConfig;
    type DataApi = data_api::BitcoinDataApi;
    type IndexerApi = indexer_api::BitcoinIndexerApi;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::<BitcoinTypes>::builder()
        .call_api(call_api::BitcoinCallApi::default())
        .custom_configuration_from_arg()
        .construction_api(construction_api::BitcoinConstructionApi::default())
        .data_api(data_api::BitcoinDataApi::default())
        .indexer_api(indexer_api::BitcoinIndexerApi::default())
        .build();

    serve!(server, BitcoinTypes,)
}
