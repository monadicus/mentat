//! The module defines the `BlockIdentifier`.

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
