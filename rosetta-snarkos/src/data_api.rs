use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    axum::{async_trait, Json},
    errors::MentatError,
    requests::*,
    responses::*,
    serde_json,
    server::RpcCaller,
    tracing,
};

use crate::{
    jsonrpc_call,
    request::SnarkosJrpc,
    responses::{common::SnarkosTransactions, data::*, Response},
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
            Ok(Json(
                jsonrpc_call!("getblock", vec![block_id], rpc_caller, BlockResult).into(),
            ))
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
        let first = jsonrpc_call!(
            "gettransaction",
            vec![data.block_identifier.hash],
            rpc_caller,
            GetTransactionResult
        );
        tracing::debug!("first {:#?}", first);
        let second = jsonrpc_call!(
            "getblocktransactions",
            vec![data.block_identifier.index],
            rpc_caller,
            SnarkosTransactions
        );
        Ok(Json(first + second))
    }

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolResponse> {
        Ok(Json(
            jsonrpc_call!(
                "getmemorypool",
                Vec::<()>::new(),
                rpc_caller,
                GetMemoryPoolResult
            )
            .into(),
        ))
    }
}
