//! The module defines the `BlockIdentifier`.

use from_tuple::FromTuple;

use super::*;

/// The [`BlockIdentifier`] uniquely identifies a block in a particular network.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq, Unchecked)]
#[serde(default)]
pub struct UncheckedBlockIdentifier {
    /// This is also known as the block height.
    #[unchecked(usize)]
    pub index: isize,
    /// The block hash..
    pub hash: String,
}
