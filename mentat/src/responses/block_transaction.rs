//! The module defines the `BlockTransactionResponse` request

use super::*;

/// A `BlockTransactionResponse` contains information about a block transaction.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BlockTransactionResponse {
    /// `Transaction`
    pub transaction: Transaction,
}
