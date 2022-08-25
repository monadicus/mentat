//! The module defines the `OperationIdentifier`.

use std::mem::size_of_val;

use super::*;

/// The [`OperationIdentifier`] uniquely identifies an operation within a
/// transaction.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct OperationIdentifier {
    /// The operation index is used to ensure each operation has a unique
    /// identifier within a transaction. This index is only relative to the
    /// transaction and NOT GLOBAL. The operations in each transaction should
    /// start from index 0. To clarify, there may not be any notion of an
    /// operation index in the blockchain being described.
    pub index: i64,
    /// Some blockchains specify an operation index that is essential for client
    /// use. For example, Bitcoin uses a `network_index` to identify which UTXO
    /// was used in a transaction. `network_index` should not be populated if
    /// there is no notion of an operation index in a blockchain (typically most
    /// account-based blockchains).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_index: Option<i64>,
}

impl From<i64> for OperationIdentifier {
    fn from(index: i64) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }
}

impl From<(i64, i64)> for OperationIdentifier {
    fn from((index, net_index): (i64, i64)) -> Self {
        Self {
            index,
            network_index: Some(net_index),
        }
    }
}

impl From<(i64, Option<i64>)> for OperationIdentifier {
    fn from((index, net_index): (i64, Option<i64>)) -> Self {
        Self {
            index,
            network_index: net_index,
        }
    }
}

impl EstimateSize for OperationIdentifier {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + self
                .network_index
                .map(|i| size_of_val(&i))
                .unwrap_or_default()
    }
}
