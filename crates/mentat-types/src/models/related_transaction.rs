//! The module defines the `RelatedTransaction` model.

use super::*;

/// The [`RelatedTransaction`] allows implementations to link together multiple
/// transactions. An unpopulated network identifier indicates that the related
/// transaction is on the same network.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedRelatedTransaction {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub network_identifier: Option<NetworkIdentifier>,
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
    /// [`Direction`] Used by `RelatedTransaction` to indicate the direction of
    /// the relation (i.e. cross-shard/cross-network sends may reference
    /// backward to an earlier transaction and async execution may reference
    /// forward). Can be used to indicate if a transaction relation is from
    /// child to parent or the reverse.
    #[unchecked(option_enum)]
    pub direction: UncheckedDirection,
}

impl Sortable for UncheckedRelatedTransaction {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.network_identifier = new.network_identifier.map(|ni| ni.sort());
        new
    }
}

impl EstimateSize for RelatedTransaction {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + estimated_option_size(&self.network_identifier)
            + self.transaction_identifier.estimated_size()
            + size_of_val(&self.direction)
    }
}
