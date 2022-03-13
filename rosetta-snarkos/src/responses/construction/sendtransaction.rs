use mentat::{
    api::MentatResponse, identifiers::TransactionIdentifier,
    responses::TransactionIdentifierResponse, IndexMap, Json,
};

use super::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
pub struct SendTransactionResponse {
    pub jsonrpc: String,
    pub result: String,
    pub id: String,
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
