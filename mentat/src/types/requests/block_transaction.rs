//! The module defines the `BlockTransactionRequest` request.

use from_tuple::FromTuple;
use mentat_macros::Nullable;

use super::*;

/// A [`BlockRequest`] is utilized to make a block request on the `/block`
/// endpoint.
#[derive(Debug, Default, Deserialize, FromTuple, Serialize, Nullable)]
#[serde(default)]
pub struct NullableBlockTransactionRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<BlockIdentifier>,
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
}
