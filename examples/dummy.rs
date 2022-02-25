use mentat::server::Server;
use std::net::Ipv4Addr;

#[rocket::main]
async fn main() {
    let server = Server::default();
    server.serve(Ipv4Addr::new(127, 0, 0, 1), 3030).await;
}
