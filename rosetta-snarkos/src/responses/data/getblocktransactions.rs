use crate::responses::common::SnarkosTransactions;

use super::*;

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetBlockTransactionsResponse {
    pub jsonrpc: String,
    pub result: SnarkosTransactions,
    pub id: String,
}
