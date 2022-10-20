//! The module defines the `BlockIdentifier`.

use from_tuple::FromTuple;

use super::*;

/// The [`BlockIdentifier`] uniquely identifies a block in a particular network.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedBlockIdentifier {
    /// This is also known as the block height.
    #[unchecked(usize)]
    pub index: isize,
    /// The block hash..
    pub hash: String,
}

impl Sortable for BlockIdentifier {
    fn sort(&self) -> Self {
        self.clone()
    }
}

impl EstimateSize for BlockIdentifier {
    fn estimated_size(&self) -> usize {
        size_of_val(self) + size_of_val(self.hash.as_str())
    }
}
