use mentat::{
    identifiers::TransactionIdentifier,
    responses::TransactionIdentifierResponse,
    IndexMap,
};

use super::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
pub struct SendTransactionResponse {
    _jsonrpc: String,
    result: String,
    _id: String,
}

impl From<SendTransactionResponse> for TransactionIdentifierResponse {
    fn from(response: SendTransactionResponse) -> TransactionIdentifierResponse {
        TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier {
                hash: response.result,
            },
            metadata: IndexMap::new(),
        }
    }
}
