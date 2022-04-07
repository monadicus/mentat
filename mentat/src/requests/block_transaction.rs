//! The module defines the `BlockTransactionRequest` request.

use super::*;

/// A `BlockRequest` is utilized to make a block request on the `/block`
/// endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BlockTransactionRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// The `BlockIdentifier` uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// The `TransactionIdentifier` uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
}
