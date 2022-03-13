use super::*;

/// A BlockTransactionResponse contains information about a block transaction.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockTransactionResponse {
    /// Transaction
    pub transaction: Transaction,
}
