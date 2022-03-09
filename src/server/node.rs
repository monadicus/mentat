#[async_trait::async_trait]
pub trait NodeRunner: Send + Sync {
    async fn start_node(
        &self,
        _address: String,
        mut _cmd: std::process::Command,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Default)]
pub struct DummyNode;

#[async_trait::async_trait]
impl NodeRunner for DummyNode {}
