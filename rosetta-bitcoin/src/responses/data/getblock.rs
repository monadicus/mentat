use crate::responses::common::BitcoinTransaction;

use super::*;
use mentat::{
    api::MentatResponse, identifiers::BlockIdentifier, models::Block, responses::BlockResponse,
    IndexMap, Json,
};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetBlockResponse {
    hash: String,
    // confirmations: usize,
    height: u64,
    version: usize,
    // versionHex: String,
    merkleroot: String,
    time: u64,
    mediantime: u64,
    nonce: usize,
    bits: String,
    difficulty: usize,
    // chainwork: String,
    // nTx: usize,
    previousblockhash: String,
    // nextblockhash: String,
    // strippedsize: usize,
    size: usize,
    weight: usize,
    tx: Vec<BitcoinTransaction>,
}

impl From<GetBlockResponse> for MentatResponse<BlockResponse> {
    fn from(response: GetBlockResponse) -> Self {
        Ok(Json(BlockResponse {
            block: Some(Block {
                block_identifier: BlockIdentifier {
                    index: response.height,
                    hash: response.hash,
                },
                parent_block_identifier: BlockIdentifier {
                    index: response.height.saturating_sub(1),
                    hash: response.previousblockhash,
                },
                timestamp: response.time,
                transactions: response.tx.into_iter().map(|t| t.into()).collect(),
                metadata: {
                    let mut map = IndexMap::new();
                    map.insert("bits".to_string(), response.bits.into());
                    map.insert("difficulty".to_string(), response.difficulty.into());
                    map.insert("mediantime".to_string(), response.mediantime.into());
                    map.insert("merkleroot".to_string(), response.merkleroot.into());
                    map.insert("nonce".to_string(), response.nonce.into());
                    map.insert("size".to_string(), response.size.into());
                    map.insert("version".to_string(), response.version.into());
                    map.insert("weight".to_string(), response.weight.into());
                    map
                },
            }),
            other_transactions: None,
        }))
    }
}
