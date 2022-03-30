use futures::future::join_all;
use mentat::{
    api::MentatResponse,
    errors::*,
    identifiers::BlockIdentifier,
    models::Block,
    responses::BlockResponse,
    Client,
    IndexMap,
    Json,
};

use super::*;
use crate::responses::common::BitcoinTransaction;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetBlockResponse {
    pub hash: String,
    // confirmations: usize,
    pub height: u64,
    pub version: usize,
    // versionHex: String,
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: usize,
    pub bits: String,
    pub difficulty: f64,
    // chainwork: String,
    // nTx: usize,
    pub previousblockhash: String,
    // nextblockhash: String,
    // strippedsize: usize,
    pub size: usize,
    pub weight: usize,
    pub tx: Vec<BitcoinTransaction>,
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
