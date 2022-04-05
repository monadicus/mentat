use mentat::{anyhow, client::Client, requests::MetadataRequest, tokio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new("http://127.0.0.1:8080")?;
    println!(
        "{:?}",
        client.network_list(&MetadataRequest::default()).await
    );
    Ok(())
}
