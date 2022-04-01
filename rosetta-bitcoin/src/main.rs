mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod macros;
mod node;
mod request;
mod responses;

use mentat::{cache::DefaultCacheInner, mentat, server::ServerType};

#[mentat(DefaultCacheInner)]
struct MentatBitcoin;

impl ServerType for MentatBitcoin {
    type CallApi = call_api::BitcoinCallApi;
    type ConstructionApi = construction_api::BitcoinConstructionApi;
    type CustomConfig = node::NodeConfig;
    type DataApi = data_api::BitcoinDataApi;
    type IndexerApi = indexer_api::BitcoinIndexerApi;
}
