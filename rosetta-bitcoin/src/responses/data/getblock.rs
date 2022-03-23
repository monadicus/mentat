use super::*;
use crate::responses::common::BitcoinTransaction;
use futures::future::join_all;
use mentat::errors::*;
use mentat::{
    api::MentatResponse, identifiers::BlockIdentifier, models::Block, responses::BlockResponse,
    Client, IndexMap, Json,
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

impl GetBlockResponse {
    pub async fn into_block_response(self, client: &Client) -> MentatResponse<BlockResponse> {
        Ok(Json(BlockResponse {
            block: Some(Block {
                transactions: join_all(
                    self.tx
                        .iter()
                        .enumerate()
                        .map(|(i, tx)| tx.into_transaction(i, client)),
                )
                .await
                .into_iter()
                .collect::<Result<_, _>>()?,
                block_identifier: BlockIdentifier {
                    index: self.height,
                    hash: self.hash,
                },
                parent_block_identifier: BlockIdentifier {
                    index: self.height.saturating_sub(1),
                    hash: self.previousblockhash,
                },
                timestamp: self.time * 1000,
                metadata: {
                    let mut map = IndexMap::new();
                    map.insert("bits".to_string(), self.bits.into());
                    map.insert("difficulty".to_string(), self.difficulty.into());
                    map.insert("mediantime".to_string(), self.mediantime.into());
                    map.insert("merkleroot".to_string(), self.merkleroot.into());
                    map.insert("nonce".to_string(), self.nonce.into());
                    map.insert("size".to_string(), self.size.into());
                    map.insert("version".to_string(), self.version.into());
                    map.insert("weight".to_string(), self.weight.into());
                    map
                },
            }),
            other_transactions: None,
        }))
    }
}
