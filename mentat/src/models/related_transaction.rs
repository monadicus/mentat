//! The module defines the `RelatedTransaction` model.

use super::*;

/// The `RelatedTransaction` allows implementations to link together multiple
/// transactions. An unpopulated network identifier indicates that the related
/// transaction is on the same network.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelatedTransaction {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// The `TransactionIdentifier` uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
    /// Direction Used by `RelatedTransaction` to indicate the direction of
    /// the relation (i.e. cross-shard/cross-network sends may reference
    /// backward to an earlier transaction and async execution may reference
    /// forward). Can be used to indicate if a transaction relation is from
    /// child to parent or the reverse.
    pub direction: Direction,
}
