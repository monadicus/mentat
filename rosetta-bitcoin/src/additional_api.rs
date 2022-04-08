use mentat::{
    api::{AdditionalApi, MentatResponse},
    async_trait,
    serde_json::{self, Value},
    server::RpcCaller,
};

use crate::{jsonrpc_call, request::BitcoinJrpc, responses::Response};

#[derive(Clone, Default)]
pub struct BitcoinAdditionalApi;

#[async_trait]
impl AdditionalApi for BitcoinAdditionalApi {
    async fn check_node_status(&self, rpc_caller: RpcCaller) -> MentatResponse<Value> {
        jsonrpc_call!("/getmemoryinfo", Vec::<()>::new(), rpc_caller, Value)
    }
}
