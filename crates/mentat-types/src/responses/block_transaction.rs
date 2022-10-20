//! The module defines the `BlockTransactionResponse` request

use super::*;

/// A [`BlockTransactionResponse`] contains information about a block
/// transaction.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedBlockTransactionResponse {
    /// [`Transaction`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<UncheckedTransaction>,
}
