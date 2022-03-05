use mentat::server::Server;
use std::net::Ipv4Addr;

mod data_api;

#[rocket::main]
async fn main() {
    let mut server = Server::new();
    server.with_data_api(data_api::SnarkosDataApi{});
    server.serve(Ipv4Addr::new(127, 0, 0, 1), 3030).await;
}
