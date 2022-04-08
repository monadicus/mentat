use mentat::{
    api::{AdditionalApi, MentatResponse},
    async_trait,
    serde_json::{json, Value},
    server::RpcCaller,
    Json,
};

#[derive(Clone, Default)]
pub struct SnarkosAdditionalApi;

#[async_trait]
impl AdditionalApi for SnarkosAdditionalApi {
    async fn check_node_status(&self, _rpc_caller: RpcCaller) -> MentatResponse<Value> {
        Ok(Json(json!("Unknown")))
    }
}
