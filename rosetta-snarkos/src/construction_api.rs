use mentat::{
    api::{Caller, CallerConstructionApi, ConstructionApi, MentatResponse},
    async_trait,
    requests::ConstructionSubmitRequest,
    responses::TransactionIdentifierResponse,
    Client,
};

use crate::{
    jsonrpc_call,
    responses::{construction::*, Response},
};

use super::SnarkosJrpc;

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[async_trait]
impl CallerConstructionApi for SnarkosConstructionApi {}

#[async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        client: Client,
    ) -> MentatResponse<TransactionIdentifierResponse> {
        jsonrpc_call!(
            @ret
            "sendtransaction",
            vec![data.signed_transaction],
            client,
            SendTransactionResponse
        )
    }
}
