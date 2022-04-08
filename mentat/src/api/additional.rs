use serde_json::Value;

use super::*;

#[axum::async_trait]
pub trait AdditionalApi: Clone + Default {
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

    async fn check_node_status(&self, rpc_caller: &RpcCaller) -> Result<Value>;

    async fn check_cache_status(&self, _rpc_caller: &RpcCaller) -> Result<Option<Value>> {
        Ok(None)
    }
}
