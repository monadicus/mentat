use super::*;

/// EventsBlocksResponse contains an ordered collection of BlockEvents and the max retrievable sequence.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct EventsBlocksResponse {
    /// max_sequence is the maximum available sequence number to fetch.
    pub max_sequence: u64,
    /// events is an array of BlockEvents indicating the order to add and remove blocks to maintain a canonical view of blockchain state. Lightweight clients can use this event stream to update state without implementing their own block syncing logic.
    pub events: Vec<BlockEvent>,
}
