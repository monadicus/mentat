use mentat::{
    api::MentatResponse,
    identifiers::TransactionIdentifier,
    responses::MempoolResponse,
    Json,
};

use super::*;

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Transition {
    _ciphertexts: Vec<String>,
    _ciphertext_ids: Vec<String>,
    _commitments: Vec<String>,
    _proof: String,
    _serial_numbers: Vec<String>,
    _transition_id: String,
    _value_balance: i32,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct GetMemoryPoolResult {
    _inner_circuit_id: String,
    _ledger_root: String,
    transaction_id: String,
    _transitions: Vec<Transition>,
}

impl From<GetMemoryPoolResult> for TransactionIdentifier {
    fn from(result: GetMemoryPoolResult) -> TransactionIdentifier {
        TransactionIdentifier {
            hash: result.transaction_id,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetMemoryPoolResponse {
    _jsonrpc: String,
    result: Vec<GetMemoryPoolResult>,
    _id: String,
}

impl From<GetMemoryPoolResponse> for MentatResponse<MempoolResponse> {
    fn from(response: GetMemoryPoolResponse) -> MentatResponse<MempoolResponse> {
        Ok(Json(MempoolResponse {
            transaction_identifiers: response.result.into_iter().map(|r| r.into()).collect(),
        }))
    }
}
