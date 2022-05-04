use mentat::{
    identifiers::BlockIdentifier,
    indexmap::IndexMap,
    models::Block,
    responses::BlockResponse,
    serde_json::Value,
};

use super::*;
use crate::responses::common::SnarkosTransactions;

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

// #[derive(Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// struct Proof {
//     _hiding: String,
// }

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
struct Header {
    metadata: Metadata,
    // _nonce: String,
    // _previous_ledger_root: String,
    // _proof: Proof,
    // _transactions_root: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BlockResult {
    block_hash: String,
    header: Header,
    previous_block_hash: String,
    transactions: SnarkosTransactions,
}

impl From<BlockResult> for BlockResponse {
    fn from(result: BlockResult) -> Self {
        BlockResponse {
            block: Some(Block {
                block_identifier: BlockIdentifier {
                    index: result.header.metadata.height,
                    hash: result.block_hash,
                },
                parent_block_identifier: BlockIdentifier {
                    index: result.header.metadata.height.saturating_sub(1),
                    hash: result.previous_block_hash,
                },
                timestamp: result.header.metadata.timestamp,
                transactions: result.transactions.into(),
                metadata: result.header.metadata.into(),
            }),
            other_transactions: None,
        }
    }
}
