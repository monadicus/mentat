use mentat::{
    api::MentatResponse,
    axum::Json,
    identifiers::TransactionIdentifier,
    indexmap::IndexMap,
    responses::TransactionIdentifierResponse,
};

use super::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
pub struct SendTransactionResponse {
    _jsonrpc: String,
    result: String,
    _id: String,
}

impl From<SendTransactionResponse> for MentatResponse<TransactionIdentifierResponse> {
    fn from(response: SendTransactionResponse) -> MentatResponse<TransactionIdentifierResponse> {
        Ok(Json(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier {
                hash: response.result,
            },
            metadata: IndexMap::new(),
        }))
    }
}
