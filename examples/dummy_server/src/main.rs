use std::net::Ipv4Addr;

use mentat::{
    serve,
    server::{DummyNode, Server},
    tokio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::default();
    serve!(
        server,
        Ipv4Addr::new(127, 0, 0, 1),
        3030,
        Box::new(DummyNode::default()),
    )
}
