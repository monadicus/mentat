use mentat::{
    api::{CallerConstructionApi, ConstructionApi, MentatResponse},
    axum::{async_trait, Json},
    requests::ConstructionSubmitRequest,
    responses::TransactionIdentifierResponse,
    serde_json,
    server::RpcCaller,
    tracing,
    Caller,
};

use crate::{
    jsonrpc_call,
    request::SnarkosJrpc,
    responses::{construction::*, Response},
};

#[derive(Clone, Default)]
pub struct SnarkosConstructionApi;

#[async_trait]
impl CallerConstructionApi for SnarkosConstructionApi {}

#[async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<TransactionIdentifierResponse> {
        Ok(Json(
            jsonrpc_call!(
                "sendtransaction",
                vec![data.signed_transaction],
                rpc_caller,
                SendTransactionResult
            )
            .into(),
        ))
    }
}
