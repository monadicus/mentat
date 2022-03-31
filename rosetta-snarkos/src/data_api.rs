use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    async_trait,
    errors::MentatError,
    requests::*,
    responses::*,
    server::RpcCaller,
};

use super::SnarkosJrpc;
use crate::{
    jsonrpc_call,
    responses::{data::*, Response},
};

#[derive(Clone, Default)]
pub struct SnarkosDataApi;

#[async_trait]
impl CallerDataApi for SnarkosDataApi {}

#[async_trait]
impl DataApi for SnarkosDataApi {
    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockResponse> {
        if let Some(block_id) = data.block_identifier.index {
            jsonrpc_call!(@ret "getblock", vec![block_id], rpc_caller, GetBlockResponse)
        } else {
            Err(MentatError::from("wtf"))
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockTransactionResponse> {
        let first = jsonrpc_call!(@res "gettransaction", vec![data.block_identifier.hash], rpc_caller, GetTransactionResponse);
        let second = jsonrpc_call!(@res "getblocktransactions", vec![data.block_identifier.index], rpc_caller, GetBlockTransactionsResponse);
        first + second
    }

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolResponse> {
        let data: Vec<u8> = Vec::new();
        jsonrpc_call!(@ret "getmemorypool", data, rpc_caller, GetMemoryPoolResponse)
    }
}
