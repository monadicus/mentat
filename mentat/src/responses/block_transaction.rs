use super::*;

/// A BlockTransactionResponse contains information about a block transaction.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BlockTransactionResponse {
    /// Transaction
    pub transaction: Transaction,
}
