use mentat::{
    identifiers::{AccountIdentifier, CoinIdentifier, OperationIdentifier, TransactionIdentifier},
    models::{Amount, CoinAction, CoinChange, Currency, Operation, Transaction},
    serde_json::{json, Value},
    IndexMap,
};

use super::*;

// TODO there are different versions of vin...fml

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct BitcoinVin {
//     coinbase: String,
//     sequence: usize,
// }

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinScriptSig {
    asm: String,
    hex: String,
}
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinVin {
    txid: String,
    vout: usize,
    scriptSig: BitcoinScriptSig,
    sequence: usize,
    txinwitness: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
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

// fn into_operation(vin: BitcoinVin, vout: BitcoinVout, txid: String, index: u64) -> [Operation; 2] {
//     [
//         Operation {
//             operation_identifier: OperationIdentifier {
//                 index: index * 2,
//                 network_index: Some(vout.n),
//             },
//             related_operations: None,
//             type_: "COINBASE".to_string(),
//             status: Some("SUCCESS".to_string()),
//             account: None,
//             amount: None,
//             coin_change: None,
//             metadata: {
//                 let mut map = IndexMap::new();
//                 map.insert("coinbase".to_string(), vin.coinbase.into());
//                 map.insert("sequence".to_string(), vin.sequence.into());
//                 map
//             },
//         },
//         Operation {
//             operation_identifier: OperationIdentifier {
//                 index: index * 2 + 1,
//                 network_index: Some(vout.n),
//             },
//             related_operations: None,
//             type_: "OUTPUT".to_string(),
//             status: Some("SUCCESS".to_string()),
//             account: Some(AccountIdentifier {
//                 address: vout.scriptPubKey.hex.clone(),
//                 decimals: None,
//                 metadata: IndexMap::new(),
//             }),
//             amount: Some(Amount {
//                 value: vout.value.to_string(),
//                 currency: Currency {
//                     symbol: "BTC".to_string(),
//                     decimals: 8,
//                     metadata: IndexMap::new(),
//                 },
//                 metadata: IndexMap::new(),
//             }),
//             coin_change: Some(CoinChange {
//                 coin_identifier: CoinIdentifier { identifier: txid },
//                 coin_action: CoinAction::CoinCreated,
//             }),
//             metadata: {
//                 let mut inner_map: IndexMap<String, Value> = IndexMap::new();
//                 inner_map.insert("asm".to_string(), vout.scriptPubKey.asm.into());
//                 inner_map.insert("hex".to_string(), vout.scriptPubKey.hex.into());
//                 inner_map.insert("type".to_string(), vout.scriptPubKey._type.into());
//                 let mut map = IndexMap::new();
//                 map.insert("scriptPubKey".to_string(), json!(inner_map));
//                 map
//             },
//         },
//     ]
// }

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinTransaction {
    txid: String,
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

impl From<BitcoinTransaction> for Transaction {
    fn from(transaction: BitcoinTransaction) -> Self {
        Transaction {
            transaction_identifier: TransactionIdentifier {
                hash: transaction.hash,
            },
            operations: transaction
                .vin
                .into_iter()
                .zip(transaction.vout)
                .enumerate()
                .flat_map(|(i, (vin, vout))| {
                    into_operation(vin, vout, transaction.txid.clone(), i as u64)
                })
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
        }
    }
}
