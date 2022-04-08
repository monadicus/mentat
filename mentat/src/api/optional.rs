//! Houses the traits for any additional API endpoints.
//! These traits are easily overridable for custom
//! implementations.

use axum::async_trait;
use serde_json::{json, Value};

use super::*;

#[axum::async_trait]
/// The `OptionalApi` Trait.
pub trait OptionalApi: Clone + Default {
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
    async fn check_node_status(&self, _rpc_caller: &RpcCaller) -> Result<Value>;

    /// A default implementation for providing a cache status check.
    async fn check_cache_status(&self, _rpc_caller: &RpcCaller) -> Result<Option<Value>> {
        Ok(None)
    }
}

#[derive(Clone, Default)]
pub struct UnimplementedOptionalApi;

#[async_trait]
impl OptionalApi for UnimplementedOptionalApi {
    async fn health(
        &self,
        _caller: Caller,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<HealthCheckResponse> {
        MentatError::not_implemented()
    }

    /// A method for providing a node status check.
    async fn check_node_status(&self, _rpc_caller: &RpcCaller) -> Result<Value> {
        Ok(json!("unknown"))
    }

    /// A default implementation for providing a cache status check.
    async fn check_cache_status(&self, _rpc_caller: &RpcCaller) -> Result<Option<Value>> {
        Ok(None)
    }
}
