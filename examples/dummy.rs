use mentat::server::Server;


#[tokio::main]
async fn main() {
    let server = Server::default();
    server.serve(([127, 0, 0, 1], 3030)).await;
}