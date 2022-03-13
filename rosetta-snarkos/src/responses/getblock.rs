use super::*;
use mentat::{
    identifiers::{BlockIdentifier, OperationIdentifier, TransactionIdentifier},
    models::{Amount, Block, Currency, Operation, Transaction},
    responses::BlockResponse,
    serde_json::Value,
    IndexMap,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
pub struct SnarkOSEvent {
    pub id: u64,
    pub index: u64,
    pub record_view_key: String,
}

impl From<SnarkOSEvent> for OperationIdentifier {
    fn from(event: SnarkOSEvent) -> Self {
        Self {
            index: event.index,
            network_index: Some(event.id),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "mentat::serde")]
pub struct SnarkOSTransitions {
    pub ciphertexts: Vec<String>,
    pub commitments: Vec<String>,
    pub events: Vec<SnarkOSEvent>,
    pub proof: String,
    pub serial_numbers: Vec<String>,
    pub transition_id: String,
    pub value_balance: i32,
}

impl From<SnarkOSTransitions> for Operation {
    fn from(transition: SnarkOSTransitions) -> Self {
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
pub struct SnarkOSTransaction {
    pub inner_circuit_id: String,
    pub ledger_root: String,
    pub transaction_id: String,
    pub transitions: Vec<SnarkOSTransitions>,
}

impl From<SnarkOSTransaction> for Transaction {
    fn from(transaction: SnarkOSTransaction) -> Self {
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
pub struct SnarkOSTransactions {
    transactions: Vec<SnarkOSTransaction>,
}

impl Into<Vec<Transaction>> for SnarkOSTransactions {
    fn into(self) -> Vec<Transaction> {
        self.transactions.into_iter().map(|t| t.into()).collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkOSMetadata {
    pub cumulative_weight: u64,
    pub difficulty_target: u64,
    pub height: u64,
    pub timestamp: u64,
}

impl From<SnarkOSMetadata> for IndexMap<String, Value> {
    fn from(metadata: SnarkOSMetadata) -> Self {
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
pub struct SnarkOSProof {
    pub hiding: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkOSHeader {
    pub metadata: SnarkOSMetadata,
    pub nonce: String,
    pub previous_ledger_root: String,
    pub proof: SnarkOSProof,
    pub transactions_root: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkOSBlockResult {
    pub block_hash: String,
    pub header: SnarkOSHeader,
    pub previous_block_hash: String,
    pub transactions: SnarkOSTransactions,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkOSBlockResponse {
    pub jsonrpc: String,
    pub result: SnarkOSBlockResult,
    pub id: String,
}

impl From<SnarkOSBlockResponse> for BlockResponse {
    fn from(response: SnarkOSBlockResponse) -> Self {
        BlockResponse {
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
            // TODO: snarkos doesn't give anything?
            other_transactions: None,
        }
    }
}
