use super::*;
use mentat::{
    api::MentatResponse,
    identifiers::{BlockIdentifier, OperationIdentifier, TransactionIdentifier},
    models::{Amount, Block, Currency, Operation, Transaction},
    responses::BlockResponse,
    serde_json::Value,
    IndexMap, Json,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
struct Event {
    id: u64,
    index: u64,
    _record_view_key: String,
}

impl From<Event> for OperationIdentifier {
    fn from(event: Event) -> Self {
        Self {
            index: event.index,
            network_index: Some(event.id),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
struct Transitions {
    _ciphertexts: Vec<String>,
    _commitments: Vec<String>,
    events: Vec<Event>,
    _proof: String,
    _serial_numbers: Vec<String>,
    _transition_id: String,
    value_balance: i32,
}

impl From<Transitions> for Operation {
    fn from(transition: Transitions) -> Self {
        Self {
            // TODO: HOW AM I SUPPOSED TA!!!!?????
            operation_identifier: OperationIdentifier {
                index: todo!(),
                network_index: todo!(),
            },
            related_operations: Some(transition.events.into_iter().map(|e| e.into()).collect()),
            type_: todo!(),
            status: None,
            account: None,
            amount: Some(Amount {
                value: transition.value_balance.to_string(),
                currency: Currency {
                    symbol: "ALEO".to_string(),
                    decimals: 18,
                    metadata: IndexMap::new(),
                },
                metadata: IndexMap::new(),
            }),
            coin_change: None,
            metadata: IndexMap::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct GetBlockTransaction {
    _inner_circuit_id: String,
    _ledger_root: String,
    transaction_id: String,
    transitions: Vec<Transitions>,
}

impl From<GetBlockTransaction> for Transaction {
    fn from(transaction: GetBlockTransaction) -> Self {
        Transaction {
            transaction_identifier: TransactionIdentifier {
                hash: transaction.transaction_id,
            },
            operations: transaction
                .transitions
                .into_iter()
                .map(|t| t.into())
                .collect(),
            related_transactions: None,
            // TODO: Size and locktime????
            metadata: IndexMap::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Transactions {
    transactions: Vec<GetBlockTransaction>,
}

impl Into<Vec<Transaction>> for Transactions {
    fn into(self) -> Vec<Transaction> {
        self.transactions.into_iter().map(|t| t.into()).collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Metadata {
    cumulative_weight: u64,
    difficulty_target: u64,
    height: u64,
    timestamp: u64,
}

impl From<Metadata> for IndexMap<String, Value> {
    fn from(metadata: Metadata) -> Self {
        let mut map = IndexMap::new();
        map.insert(
            "cumulative_weight".to_string(),
            metadata.cumulative_weight.into(),
        );
        map.insert(
            "difficulty_target".to_string(),
            metadata.difficulty_target.into(),
        );
        map.insert("height".to_string(), metadata.height.into());
        map.insert("timestamp".to_string(), metadata.timestamp.into());
        map
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Proof {
    _hiding: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Header {
    metadata: Metadata,
    _nonce: String,
    _previous_ledger_root: String,
    _proof: Proof,
    _transactions_root: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct BlockResult {
    block_hash: String,
    header: Header,
    previous_block_hash: String,
    transactions: Transactions,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetBlockResponse {
    _jsonrpc: String,
    result: BlockResult,
    _id: String,
}

impl From<GetBlockResponse> for MentatResponse<BlockResponse> {
    fn from(response: GetBlockResponse) -> Self {
        Ok(Json(BlockResponse {
            block: Some(Block {
                block_identifier: BlockIdentifier {
                    // TODO: is this correct?
                    index: response.result.header.metadata.height,
                    hash: response.result.block_hash,
                },
                parent_block_identifier: BlockIdentifier {
                    // TODO: is this correct?
                    index: response.result.header.metadata.height.saturating_sub(1),
                    hash: response.result.previous_block_hash,
                },
                timestamp: response.result.header.metadata.timestamp,
                transactions: response.result.transactions.into(),
                metadata: response.result.header.metadata.into(),
            }),
            // TODO:  doesn't give anything?
            other_transactions: None,
        }))
    }
}
