use crate::responses::common::BitcoinTransaction;

use super::*;
use mentat::{
    api::MentatResponse,
    identifiers::{
        AccountIdentifier, BlockIdentifier, CoinIdentifier, OperationIdentifier,
        TransactionIdentifier,
    },
    models::{Block, CoinAction, CoinChange, Operation, Transaction},
    responses::BlockResponse,
    serde_json, IndexMap, Json,
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
                transactions: response
                    .tx
                    .into_iter()
                    .enumerate()
                    .map(|(it, transaction)| Transaction {
                        transaction_identifier: TransactionIdentifier {
                            hash: transaction.hash.clone(),
                        },
                        operations: transaction
                            .vin
                            .into_iter()
                            .enumerate()
                            .map(|(io, vin)| Operation {
                                operation_identifier: OperationIdentifier {
                                    index: io as u64,
                                    network_index: Some(vin.vout.unwrap_or(0)),
                                },
                                related_operations: None,
                                type_: if it == 0 && io == 0 {
                                    "COINBASE"
                                } else {
                                    "INPUT"
                                }
                                .to_string(),
                                status: Some(String::from("SUCCESS")),
                                account: vin.scriptSig.map(|s| AccountIdentifier {
                                    address: s.hex,
                                    decimals: None,
                                    metadata: IndexMap::new(),
                                }),
                                amount: todo!(),
                                coin_change: vin.txid.map(|id| CoinChange {
                                    coin_identifier: CoinIdentifier { identifier: id },
                                    coin_action: CoinAction::CoinSpent,
                                }),
                                metadata: {
                                    let mut map = IndexMap::new();
                                    if let Some(sig) = vin.scriptSig {
                                        map.insert(
                                            "scriptSig".to_string(),
                                            serde_json::to_value(sig).unwrap(),
                                        );
                                    }
                                    if let Some(c) = vin.coinbase {
                                        map.insert("coinbase".to_string(), c.into());
                                    }
                                    map.insert("sequence".to_string(), vin.sequence.into());
                                    map
                                },
                            })
                            .chain(transaction.vout.into_iter().enumerate().map(|(io, vout)| {
                                Operation {
                                    operation_identifier: OperationIdentifier {
                                        index: io as u64,
                                        network_index: Some(vout.n),
                                    },
                                    related_operations: None,
                                    type_: String::from("OUTPUT"),
                                    status: Some(String::from("SUCCESS")),
                                    account: Some(AccountIdentifier {
                                        address: vout.scriptPubKey.hex,
                                        decimals: None,
                                        metadata: IndexMap::new(),
                                    }),
                                    amount: todo!(),
                                    coin_change: Some(CoinChange {
                                        coin_identifier: CoinIdentifier {
                                            identifier: transaction.hash,
                                        },
                                        coin_action: CoinAction::CoinCreated,
                                    }),
                                    metadata: {
                                        let mut map = IndexMap::new();
                                        map.insert(
                                            "scriptSig".to_string(),
                                            serde_json::to_value(vout.scriptPubKey).unwrap(),
                                        );
                                        map
                                    },
                                }
                            }))
                            .collect(),
                        related_transactions: None,
                        metadata: {
                            let mut map = IndexMap::new();
                            map.insert("size".to_string(), transaction.size.into());
                            map.insert("version".to_string(), transaction.version.into());
                            map.insert("vsize".to_string(), transaction.vsize.into());
                            map.insert("weight".to_string(), transaction.weight.into());
                            map
                        },
                    })
                    .collect(),
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
