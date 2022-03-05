use mentat::server::Server;
use std::net::Ipv4Addr;

mod construction_api;
mod data_api;
mod indexer_api;

#[rocket::main]
async fn main() {
    let mut server = Server::new();
    server.with_construction_api(construction_api::SnarkosConstructionApi::default());
    server.with_data_api(data_api::SnarkosDataApi::default());
    server.with_indexer_api(indexer_api::SnarkosIndexerApi::default());
    server.serve(Ipv4Addr::new(127, 0, 0, 1), 3030).await;
}
