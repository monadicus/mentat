use futures::future::join_all;
use mentat::{
    errors::MentatError,
    identifiers::{AccountIdentifier, CoinIdentifier, OperationIdentifier, TransactionIdentifier},
    models::{Amount, CoinAction, CoinChange, Currency, Operation, Transaction},
    serde::Serialize,
    serde_json::{self, json},
    server::RpcCaller,
    IndexMap,
};

use super::*;
use crate::{
    jsonrpc_call,
    request::{trim_hash, BitcoinJrpc},
    responses::Response,
};

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
    pub txid: Option<String>,
    pub vout: Option<u64>,
    pub scriptSig: Option<BitcoinScriptSig>,
    pub sequence: usize,
    // txinwitness: Option<Vec<String>>,
    pub coinbase: Option<String>,
}

impl BitcoinVin {
    async fn into_operation(
        self,
        trans_idx: usize,
        vin_index: u64,
        rpc_caller: &RpcCaller,
    ) -> Result<Operation, MentatError> {
        let (account, amount) = match (&self.txid, self.vout) {
            (Some(id), Some(vout_idx)) => {
                let transaction = jsonrpc_call!(
                    "getrawtransaction",
                    vec![json!(trim_hash(id)), json!(true)],
                    rpc_caller,
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
    pub asm: String,
    pub hex: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinVout {
    pub value: f64,
    pub n: u64,
    pub scriptPubKey: BitcoinScriptPubKey,
}

impl BitcoinVout {
    fn into_operation(self, index: u64, hash: String) -> Operation {
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
    pub hash: String,
    pub version: usize,
    pub size: usize,
    pub vsize: usize,
    pub weight: usize,
    // locktime: usize,
    pub vin: Vec<BitcoinVin>,
    pub vout: Vec<BitcoinVout>,
    // hex: String,
}

impl BitcoinTransaction {
    pub async fn into_transaction(
        self,
        index: usize,
        rpc_caller: &RpcCaller,
    ) -> Result<Transaction, MentatError> {
        Ok(Transaction {
            transaction_identifier: TransactionIdentifier {
                hash: self.hash.clone(),
            },
            operations: {
                let vin_len = self.vin.len();
                let mut out: Vec<Operation> = join_all(
                    self.vin
                        .into_iter()
                        .enumerate()
                        .map(|(i, vin)| vin.into_operation(index, i as u64, rpc_caller)),
                )
                .await
                .into_iter()
                .collect::<Result<_, _>>()?;
                out.extend(
                    self.vout
                        .into_iter()
                        .enumerate()
                        .map(|(i, vout)| {
                            vout.into_operation((i + vin_len) as u64, self.hash.clone())
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
