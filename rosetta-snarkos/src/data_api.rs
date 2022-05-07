use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    axum::{async_trait, Json},
    requests::*,
    responses::*,
    server::RpcCaller,
    tracing,
};

use crate::{
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
            let result = rpc_caller
                .rpc_call::<Response<BlockResult>>(SnarkosJrpc::new("getblock", vec![block_id]))
                .await?;
            Ok(Json(result.into()))
        } else {
            Err("todo".into())
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockTransactionResponse> {
        let first = rpc_caller
            .rpc_call::<Response<GetTransactionResult>>(SnarkosJrpc::new(
                "gettransaction",
                vec![data.block_identifier.hash],
            ))
            .await?;
        let second = rpc_caller
            .rpc_call::<Response<SnarkosTransactions>>(SnarkosJrpc::new(
                "getblocktransactions",
                vec![data.block_identifier.index],
            ))
            .await?;
        tracing::debug!("first {:#?}", first);
        Ok(Json(first + second))
    }

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolResponse> {
        let result = rpc_caller
            .rpc_call::<Response<GetMemoryPoolResult>>(SnarkosJrpc::new(
                "getmemorypool",
                Vec::<()>::new(),
            ))
            .await?;
        Ok(Json(result.into()))
    }
}
