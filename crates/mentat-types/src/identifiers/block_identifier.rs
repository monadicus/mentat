//! The module defines the `BlockIdentifier`.

use from_tuple::FromTuple;

use super::*;

/// The [`BlockIdentifier`] uniquely identifies a block in a particular network.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq, Nullable)]
#[serde(default)]
pub struct NullableBlockIdentifier {
    /// This is also known as the block height.
    #[nullable(usize)]
    pub index: isize,
    /// The block hash..
    pub hash: String,
}
