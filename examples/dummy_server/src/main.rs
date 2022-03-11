use mentat::{
    server::{DummyNode, Server},
    tokio,
};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::default();
    server
        .serve(
            Ipv4Addr::new(127, 0, 0, 1),
            3030,
            Box::new(DummyNode::default()),
        )
        .await?;

    Ok(())
}
