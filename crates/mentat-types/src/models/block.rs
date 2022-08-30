//! The module defines the `Block` model.

use super::*;

/// `Block`s contain an array of [`Transaction`]s that occurred at a particular
/// [`BlockIdentifier`]. A hard requirement for blocks returned by Rosetta
/// implementations is that they MUST be inalterable: once a client has
/// requested and received a block identified by a specific [`BlockIdentifier`],
/// all future calls for that same [`BlockIdentifier`] must return the same
/// block contents.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedBlock {
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<UncheckedBlockIdentifier>,
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_block_identifier: Option<UncheckedBlockIdentifier>,
    /// The timestamp of the block in milliseconds since the Unix Epoch. The
    /// timestamp is stored in milliseconds because some blockchains produce
    /// blocks more often than once a second.
    #[unchecked(usize)]
    pub timestamp: isize,
    /// The list of [`Transaction`]s related to the block.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub transactions: Vec<Option<UncheckedTransaction>>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: IndexMap<String, Value>,
}

impl EstimateSize for Block {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + self.block_identifier.estimated_size()
            + self.parent_block_identifier.estimated_size()
            + estimated_vec_size(&self.transactions)
            + estimated_metadata_size(&self.metadata)
    }
}
