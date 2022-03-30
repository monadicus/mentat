use super::serve_exports::Configuration;

#[axum::async_trait]
pub trait NodeRunner: Send + Sync + 'static {
    type Custom: Default + Send + Sync + 'static;

    async fn start_node(
        &self,
        _config: &Configuration<Self::Custom>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
