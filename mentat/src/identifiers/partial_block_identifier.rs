use from_tuple::FromTuple;

use super::*;

/// When fetching data by BlockIdentifier, it may be possible to only specify
/// the index or hash. If neither property is specified, it is assumed that the
/// client is making a request at the current block.
#[derive(Serialize, Deserialize, Debug, Default, FromTuple)]
pub struct PartialBlockIdentifier {
    /// This is also known as the block height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl From<u64> for PartialBlockIdentifier {
    fn from(index: u64) -> Self {
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
