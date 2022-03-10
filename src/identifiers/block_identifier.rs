use super::*;

/// The block_identifier uniquely identifies a block in a particular network.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct BlockIdentifier {
    /// This is also known as the block height.
    pub index: u64,
    pub hash: String,
}
