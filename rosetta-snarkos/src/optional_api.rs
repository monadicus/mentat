use mentat::{
    api::OptionalApi,
    axum::async_trait,
    errors::Result,
    serde_json::{self, Value},
    server::RpcCaller,
    tracing,
};

use crate::{jsonrpc_call, request::SnarkosJrpc, responses::Response};

#[derive(Clone, Default)]
pub struct SnarkosOptionalApi;

#[async_trait]
impl OptionalApi for SnarkosOptionalApi {
    async fn check_node_status(&self, rpc_caller: &RpcCaller) -> Result<Value> {
        Ok(jsonrpc_call!(
            "getnodestate",
            Vec::<()>::new(),
            rpc_caller,
            Value
        ))
    }
}
