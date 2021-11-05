use super::*;

/// When fetching data by BlockIdentifier, it may be possible to only specify the index or hash. If neither property is specified, it is assumed that the client is making a request at the current block.
#[derive(Serialize, Deserialize)]
pub struct PartialBlockIdentifier {
    /// This is also known as the block height.
    pub index: Option<u64>,
    pub hash: Option<String>,
}