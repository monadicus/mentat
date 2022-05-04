use mentat::{identifiers::TransactionIdentifier, responses::MempoolResponse};

use super::*;

// #[derive(Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// struct Transition {
//     _ciphertexts: Vec<String>,
//     _ciphertext_ids: Vec<String>,
//     _commitments: Vec<String>,
//     _proof: String,
//     _serial_numbers: Vec<String>,
//     _transition_id: String,
//     _value_balance: i32,
// }

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetMemoryPoolResultItem {
    // _inner_circuit_id: String,
    // _ledger_root: String,
    transaction_id: String,
    // _transitions: Vec<Transition>,
}

impl From<GetMemoryPoolResultItem> for TransactionIdentifier {
    fn from(item: GetMemoryPoolResultItem) -> TransactionIdentifier {
        TransactionIdentifier {
            hash: item.transaction_id,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetMemoryPoolResult(Vec<GetMemoryPoolResultItem>);

impl From<GetMemoryPoolResult> for MempoolResponse {
    fn from(result: GetMemoryPoolResult) -> MempoolResponse {
        MempoolResponse {
            transaction_identifiers: result.0.into_iter().map(|r| r.into()).collect(),
        }
    }
}
