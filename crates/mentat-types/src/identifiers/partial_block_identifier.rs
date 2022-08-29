//! The module defines the `PartialBlockIdentifier`.

use from_tuple::FromTuple;

use super::*;

/// When fetching data by [`BlockIdentifier`], it may be possible to only
/// specify the index or hash. If neither property is specified, it is assumed
/// that the client is making a request at the current block.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq, Nullable)]
#[serde(default)]
pub struct NullablePartialBlockIdentifier {
    /// This is also known as the block height.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(option_usize)]
    pub index: Option<isize>,
    /// The block hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(retain)]
    pub hash: Option<String>,
}

impl From<usize> for PartialBlockIdentifier {
    fn from(index: usize) -> Self {
        Self {
            index: Some(index),
            ..Default::default()
        }
    }
}

impl From<String> for PartialBlockIdentifier {
    fn from(hash: String) -> Self {
        Self {
            hash: Some(hash),
            ..Default::default()
        }
    }
}
