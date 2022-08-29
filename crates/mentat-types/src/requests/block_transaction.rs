//! The module defines the `BlockTransactionRequest` request.

use from_tuple::FromTuple;

use super::*;

/// A [`BlockRequest`] is utilized to make a block request on the `/block`
/// endpoint.
#[derive(Debug, Default, Deserialize, FromTuple, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedBlockTransactionRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<UncheckedBlockIdentifier>,
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
}

impl
    From<(
        NetworkIdentifier,
        UncheckedBlockIdentifier,
        TransactionIdentifier,
    )> for UncheckedBlockTransactionRequest
{
    fn from(
        (network_identifier, block_identifier, transaction_identifier): (
            NetworkIdentifier,
            UncheckedBlockIdentifier,
            TransactionIdentifier,
        ),
    ) -> Self {
        Self {
            network_identifier: Some(network_identifier),
            block_identifier: Some(block_identifier),
            transaction_identifier: Some(transaction_identifier),
        }
    }
}
