//! The module defines the `OperationIdentifier`.

use super::*;

/// The [`OperationIdentifier`] uniquely identifies an operation within a
/// transaction.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Unchecked)]
#[serde(default)]
pub struct UncheckedOperationIdentifier {
    /// The operation index is used to ensure each operation has a unique
    /// identifier within a transaction. This index is only relative to the
    /// transaction and NOT GLOBAL. The operations in each transaction should
    /// start from index 0. To clarify, there may not be any notion of an
    /// operation index in the blockchain being described.
    #[unchecked(usize)]
    pub index: isize,
    /// Some blockchains specify an operation index that is essential for client
    /// use. For example, Bitcoin uses a `network_index` to identify which UTXO
    /// was used in a transaction. `network_index` should not be populated if
    /// there is no notion of an operation index in a blockchain (typically most
    /// account-based blockchains).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(option_usize)]
    pub network_index: Option<isize>,
}

impl From<usize> for OperationIdentifier {
    fn from(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }
}

impl From<(usize, usize)> for OperationIdentifier {
    fn from((index, net_index): (usize, usize)) -> Self {
        Self {
            index,
            network_index: Some(net_index),
        }
    }
}

impl From<(usize, Option<usize>)> for OperationIdentifier {
    fn from((index, net_index): (usize, Option<usize>)) -> Self {
        Self {
            index,
            network_index: net_index,
        }
    }
}
