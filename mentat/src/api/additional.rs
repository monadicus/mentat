//! Houses the traits for any additional API endpoints.
//! These traits are easily overridable for custom
//! implementations.

use serde_json::Value;

use super::*;

#[axum::async_trait]
/// The `AdditionalApi` Trait.
pub trait AdditionalApi: Clone + Default {
    /// A default implementation for providing a health check.
    async fn health(
        &self,
        caller: Caller,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<HealthCheckResponse> {
        tracing::debug!("health check!");
        Ok(Json(HealthCheckResponse {
            caller,
            msg: "Healthy!".to_string(),
            node_status: self.check_node_status(&rpc_caller).await?,
            cache_status: self.check_cache_status(&rpc_caller).await?,
        }))
    }

    /// A method for providing a node status check.
    async fn check_node_status(&self, rpc_caller: &RpcCaller) -> Result<Value>;

    /// A default implementation for providing a cache status check.
    async fn check_cache_status(&self, _rpc_caller: &RpcCaller) -> Result<Option<Value>> {
        Ok(None)
    }
}
