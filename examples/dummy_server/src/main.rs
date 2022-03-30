use mentat::{
    serve,
    server::{DummyNode, Server},
    tokio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::default();
    serve!(server, DummyNode::default(),)
}
