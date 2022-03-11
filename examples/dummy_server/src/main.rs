use mentat::server::{DummyNode, Server};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    let server = Server::default();
    server
        .serve(
            Ipv4Addr::new(127, 0, 0, 1),
            3030,
            Box::new(DummyNode::default()),
        )
        .await;
}
