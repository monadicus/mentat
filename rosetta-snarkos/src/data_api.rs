use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    axum::{async_trait, Json},
    errors::MentatError,
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
            let result: BlockResult = rpc_caller
                .rpc_call::<_, _, Response<BlockResult>>(SnarkosJrpc::new(
                    "getblock",
                    vec![block_id],
                ))
                .await?;
            Ok(Json(result.into()))
        } else {
            Err(MentatError::from("todo"))
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockTransactionResponse> {
        let first: GetTransactionResult = rpc_caller
            .rpc_call::<_, _, Response<GetTransactionResult>>(SnarkosJrpc::new(
                "gettransaction",
                vec![data.block_identifier.hash],
            ))
            .await?;
        let second: SnarkosTransactions = rpc_caller
            .rpc_call::<_, _, Response<SnarkosTransactions>>(SnarkosJrpc::new(
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
        let result: GetMemoryPoolResult = rpc_caller
            .rpc_call::<_, _, Response<GetMemoryPoolResult>>(SnarkosJrpc::new(
                "getmemorypool",
                Vec::<()>::new(),
            ))
            .await?;
        Ok(Json(result.into()))
    }
}
