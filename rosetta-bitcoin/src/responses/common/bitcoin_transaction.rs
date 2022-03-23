use futures::future::join_all;
use mentat::{
    errors::MentatError,
    identifiers::{AccountIdentifier, CoinIdentifier, OperationIdentifier, TransactionIdentifier},
    models::{Amount, CoinAction, CoinChange, Currency, Operation, Transaction},
    serde::Serialize,
    serde_json::{self, json},
    Client, IndexMap,
};

use crate::jsonrpc_call;

use super::*;

use crate::{request::BitcoinJrpc, responses::Response};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinScriptSig {
    asm: String,
    hex: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinVin {
    txid: Option<String>,
    vout: Option<u64>,
    scriptSig: Option<BitcoinScriptSig>,
    sequence: usize,
    // txinwitness: Option<Vec<String>>,
    coinbase: Option<String>,
}

impl BitcoinVin {
    async fn into_operation(
        &self,
        trans_idx: usize,
        vin_index: u64,
        client: &Client,
    ) -> Result<Operation, MentatError> {
        let (account, amount) = match (&self.txid, self.vout) {
            (Some(id), Some(vout_idx)) => {
                let transaction = jsonrpc_call!(
                    "getrawtransaction",
                    vec![json!(id), json!(2)],
                    client,
                    BitcoinTransaction
                );
                let vout = &transaction.vout[vout_idx as usize];

                let account = AccountIdentifier {
                    address: vout.scriptPubKey.hex.clone(),
                    decimals: None,
                    metadata: IndexMap::new(),
                };

                let amount = Amount {
                    value: (-(vout.value * 100000000.0) as isize).to_string(),
                    currency: Currency {
                        symbol: String::from("BTC"),
                        decimals: 8,
                        metadata: IndexMap::new(),
                    },
                    metadata: IndexMap::new(),
                };

                (Some(account), Some(amount))
            }
            _ => (None, None),
        };

        Ok(Operation {
            operation_identifier: OperationIdentifier {
                index: vin_index as u64,
                network_index: Some(self.vout.unwrap_or(0)),
            },
            related_operations: None,
            type_: if trans_idx == 0 && vin_index == 0 {
                "COINBASE"
            } else {
                "INPUT"
            }
            .to_string(),
            status: Some(String::from("SUCCESS")),
            account,
            amount,
            coin_change: self.txid.as_ref().map(|id| CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: {
                        let mut out = id.clone();
                        if let Some(vout) = self.vout {
                            out.push_str(&format!(":{}", vout));
                        }
                        out
                    },
                },
                coin_action: CoinAction::CoinSpent,
            }),
            metadata: {
                let mut map = IndexMap::new();
                if let Some(sig) = &self.scriptSig {
                    map.insert("scriptsig".to_string(), serde_json::to_value(sig).unwrap());
                }
                if let Some(c) = &self.coinbase {
                    map.insert("coinbase".to_string(), c.clone().into());
                }
                map.insert("sequence".to_string(), self.sequence.into());
                map
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinScriptPubKey {
    asm: String,
    hex: String,
    #[serde(rename = "type")]
    _type: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinVout {
    value: f64,
    n: u64,
    scriptPubKey: BitcoinScriptPubKey,
}

impl BitcoinVout {
    fn into_operation(&self, index: u64, hash: String) -> Operation {
        Operation {
            operation_identifier: OperationIdentifier {
                index,
                network_index: Some(self.n),
            },
            related_operations: None,
            type_: String::from("OUTPUT"),
            status: Some(String::from("SUCCESS")),
            account: Some(AccountIdentifier {
                address: self.scriptPubKey.hex.clone(),
                decimals: None,
                metadata: IndexMap::new(),
            }),
            amount: Some(Amount {
                value: ((self.value * 100000000.0) as isize).to_string(),
                currency: Currency {
                    symbol: String::from("BTC"),
                    decimals: 8,
                    metadata: IndexMap::new(),
                },
                metadata: IndexMap::new(),
            }),
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: format!("{}:{}", hash, self.n),
                },
                coin_action: CoinAction::CoinCreated,
            }),
            metadata: {
                let mut map = IndexMap::new();
                map.insert(
                    "scriptPubKey".to_string(),
                    serde_json::to_value(&self.scriptPubKey).unwrap(),
                );
                map
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinTransaction {
    // txid: String,
    hash: String,
    version: usize,
    size: usize,
    vsize: usize,
    weight: usize,
    // locktime: usize,
    vin: Vec<BitcoinVin>,
    vout: Vec<BitcoinVout>,
    // hex: String,
}

impl BitcoinTransaction {
    pub async fn into_transaction(
        &self,
        index: usize,
        client: &Client,
    ) -> Result<Transaction, MentatError> {
        Ok(Transaction {
            transaction_identifier: TransactionIdentifier {
                hash: self.hash.clone(),
            },
            operations: {
                let mut out: Vec<Operation> = join_all(
                    self.vin
                        .iter()
                        .enumerate()
                        .map(|(i, vin)| vin.into_operation(index, i as u64, client)),
                )
                .await
                .into_iter()
                .collect::<Result<_, _>>()?;
                out.extend(
                    self.vout
                        .iter()
                        .enumerate()
                        .map(|(i, vout)| {
                            vout.into_operation((i + self.vin.len()) as u64, self.hash.clone())
                        })
                        .collect::<Vec<_>>(),
                );
                out
            },
            related_transactions: None,
            metadata: {
                let mut map = IndexMap::new();
                map.insert("size".to_string(), self.size.into());
                map.insert("version".to_string(), self.version.into());
                map.insert("vsize".to_string(), self.vsize.into());
                map.insert("weight".to_string(), self.weight.into());
                map
            },
        })
    }
}
