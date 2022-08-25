//! The module defines the `BlockIdentifier`.

use std::mem::size_of_val;

use from_tuple::FromTuple;

use super::*;

/// The [`BlockIdentifier`] uniquely identifies a block in a particular network.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct BlockIdentifier {
    /// This is also known as the block height.
    pub index: i64,
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
