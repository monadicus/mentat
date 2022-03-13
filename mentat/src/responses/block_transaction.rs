use super::*;

/// A BlockTransactionResponse contains information about a block transaction.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockTransactionResponse {
    /// Transactions contain an array of Operations that are attributable to the
    /// same TransactionIdentifier.
    pub transaction: Vec<Transaction>,
}
