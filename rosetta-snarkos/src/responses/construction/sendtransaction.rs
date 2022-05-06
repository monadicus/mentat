use mentat::{
    identifiers::TransactionIdentifier, indexmap::IndexMap,
    responses::TransactionIdentifierResponse,
};

use super::*;

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]

pub struct SendTransactionResult(String);

impl From<SendTransactionResult> for TransactionIdentifierResponse {
    fn from(result: SendTransactionResult) -> TransactionIdentifierResponse {
        TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash: result.0 },
            metadata: IndexMap::new(),
        }
    }
}
