use crate::{
    jsonrpc_call,
    responses::{data::*, Response},
};

use super::SnarkosJrpc;

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    async_trait,
    errors::MentatError,
    requests::*,
    responses::*,
    Client,
};

#[derive(Default)]
pub struct SnarkosDataApi;

#[async_trait]
impl CallerDataApi for SnarkosDataApi {}

#[async_trait]
impl DataApi for SnarkosDataApi {
    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        client: Client,
    ) -> MentatResponse<BlockResponse> {
        if let Some(block_id) = data.block_identifier.index {
            jsonrpc_call!(@ret "getblock", vec![block_id], client, GetBlockResponse)
        } else {
            Err(MentatError::from("wtf"))
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        client: Client,
    ) -> MentatResponse<BlockTransactionResponse> {
        let first = jsonrpc_call!(@res "gettransaction", vec![data.block_identifier.hash], client, GetTransactionResponse);
        let second = jsonrpc_call!(@res "getblocktransactions", vec![data.block_identifier.index], client, GetBlockTransactionsResponse);
        first + second
    }

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        client: Client,
    ) -> MentatResponse<MempoolResponse> {
        let data: Vec<u8> = Vec::new();
        jsonrpc_call!(@ret "getmemorypool", data, client, GetMemoryPoolResponse)
    }
}
