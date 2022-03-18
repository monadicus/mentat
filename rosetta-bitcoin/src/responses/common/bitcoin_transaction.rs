use mentat::serde::Serialize;

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinScriptSig {
    pub asm: String,
    pub hex: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinVin {
    pub txid: Option<String>,
    pub vout: Option<u64>,
    pub scriptSig: Option<BitcoinScriptSig>,
    pub sequence: usize,
    pub txinwitness: Option<Vec<String>>,
    pub coinbase: Option<String>,
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

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinTransaction {
    pub txid: String,
    pub hash: String,
    pub version: usize,
    pub size: usize,
    pub vsize: usize,
    pub weight: usize,
    pub locktime: usize,
    pub vin: Vec<BitcoinVin>,
    pub vout: Vec<BitcoinVout>,
    pub hex: String,
}
