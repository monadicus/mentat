//! The module defines the `BlockIdentifier`.

use from_tuple::FromTuple;

use super::*;

/// The `block_identifier` uniquely identifies a block in a particular network.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize)]
pub struct BlockIdentifier {
    /// This is also known as the block height.
    pub index: u64,
    /// The block hash..
    pub hash: String,
}
