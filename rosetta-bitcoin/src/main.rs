use mentat::{
    server::{DummyNode, Server},
    tokio,
};

use std::{env, net::Ipv4Addr, sync::Arc};

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new();
    server.with_dyn_call_api(Arc::new(call_api::BitcoinCallApi::default()));
    server.with_dyn_construction_api(Arc::new(construction_api::BitcoinConstructionApi::default()));
    server.with_dyn_data_api(Arc::new(data_api::BitcoinDataApi::default()));
    server.with_dyn_indexer_api(Arc::new(indexer_api::BitcoinIndexerApi::default()));

    let address = env::var("ADDRESS")
        .unwrap_or_else(|_| "localhost".to_string())
        .parse()
        .unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    server
        .serve(address, port, Box::new(DummyNode::default()))
        .await?;

    Ok(())
}
