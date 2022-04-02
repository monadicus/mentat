use mentat::{
    identifiers::BlockIdentifier,
    models::{Amount, Currency},
    responses::AccountBalanceResponse,
    serde::Deserialize,
    IndexMap,
};

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct Unspents {
//     txid: String,
//     vout: usize,
//     scriptPubKey: String,
//     desc: String,
//     amount: f64,
//     height: usize,
// }

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct ScanTxOutSetResult {
    // success: bool,
    // txouts: usize,
    height: u64,
    bestblock: String,
    // unspents: Vec<Unspents>,
    total_amount: f64,
}

impl From<ScanTxOutSetResult> for AccountBalanceResponse {
    fn from(utxo: ScanTxOutSetResult) -> Self {
        AccountBalanceResponse {
            block_identifier: BlockIdentifier {
                index: utxo.height,
                hash: utxo.bestblock,
            },
            balances: vec![Amount {
                value: (utxo.total_amount * 100000000.0).to_string(),
                currency: Currency {
                    symbol: String::from("BTC"),
                    decimals: 8,
                    metadata: IndexMap::new(),
                },
                metadata: IndexMap::new(),
            }],
            metadata: IndexMap::new(),
        }
    }
}
