use mentat::{
    api::{Caller, CallerConstructionApi, ConstructionApi, MentatResponse},
    async_trait,
    requests::ConstructionSubmitRequest,
    responses::TransactionIdentifierResponse,
    server::RpcCaller,
};

use super::SnarkosJrpc;
use crate::{
    jsonrpc_call,
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
        jsonrpc_call!(
            @ret
            "sendtransaction",
            vec![data.signed_transaction],
            rpc_caller,
            SendTransactionResponse
        )
    }
}
