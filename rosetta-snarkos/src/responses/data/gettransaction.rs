use std::ops::Add;

use mentat::{api::MentatResponse, errors::MentatError, responses::BlockTransactionResponse, Json};

use crate::responses::common::SnarkosTransaction;

use super::*;

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct DecryptedRecords {
    _commitment: String,
    _owner: String,
    _payload: String,
    _program_id: String,
    _randomizer: String,
    _record_view_key: String,
    _value: i64,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Metadata {
    _block_hash: String,
    _block_height: u64,
    _block_timestamp: u64,
    transaction_index: usize,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct GetTransactionResult {
    _decrypted_records: Vec<DecryptedRecords>,
    metadata: Metadata,
    _transaction: SnarkosTransaction,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetTransactionResponse {
    _jsonrpc: String,
    result: GetTransactionResult,
    _id: String,
}

impl Add<GetBlockTransactionsResponse> for GetTransactionResponse {
    type Output = MentatResponse<BlockTransactionResponse>;

    fn add(self, other: GetBlockTransactionsResponse) -> Self::Output {
        let transaction = other
            .result
            .transactions
            .get(self.result.metadata.transaction_index)
            .cloned()
            .unwrap();
        Ok(Json(BlockTransactionResponse {
            transaction: transaction.into(),
        }))
    }
}
