use mentat::{
    api::AdditionalApi,
    axum::async_trait,
    errors::Result,
    serde_json::{self, Value},
    server::RpcCaller,
};

use crate::{jsonrpc_call, request::BitcoinJrpc, responses::Response};

#[derive(Clone, Default)]
pub struct BitcoinAdditionalApi;

#[async_trait]
impl AdditionalApi for BitcoinAdditionalApi {
    async fn check_node_status(&self, rpc_caller: &RpcCaller) -> Result<Value> {
        Ok(jsonrpc_call!(
            "/getmemoryinfo",
            Vec::<()>::new(),
            rpc_caller,
            Value
        ))
    }
}
