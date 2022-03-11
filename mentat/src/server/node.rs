#[axum::async_trait]
pub trait NodeRunner: Send + Sync + 'static {
    async fn start_node(&self, _address: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Default)]
pub struct DummyNode;

#[axum::async_trait]
impl NodeRunner for DummyNode {}
