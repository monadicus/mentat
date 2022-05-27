use mentat::{api::OptionalApi, axum::async_trait, errors::Result, server::RpcCaller};

#[derive(Clone, Default)]
pub struct SnarkosOptionalApi;

#[async_trait]
impl OptionalApi for SnarkosOptionalApi {
    async fn node_address(&self, _rpc_caller: &RpcCaller) -> Result<String> {
        Ok("unknown".to_string())
    }
}
