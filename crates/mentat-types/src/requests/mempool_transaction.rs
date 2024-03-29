//! The module defines the `MempoolTransactionRequest` request.

use from_tuple::FromTuple;

use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedMempoolTransactionRequest {
    /// [`EventsBlocksRequest`] is utilized to fetch a sequence of
    /// [`BlockEvent`]s indicating which blocks were added and removed from
    /// storage to reach the current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
}

impl From<(NetworkIdentifier, TransactionIdentifier)> for UncheckedMempoolTransactionRequest {
    fn from(
        (network_identifier, transaction_identifier): (NetworkIdentifier, TransactionIdentifier),
    ) -> Self {
        Self {
            network_identifier: Some(network_identifier),
            transaction_identifier: Some(transaction_identifier),
        }
    }
}
