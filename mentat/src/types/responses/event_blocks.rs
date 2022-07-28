//! The module defines the `EventsBlocksResponse` response.

use super::*;

/// `EventsBlocksResponse` contains an ordered collection of [`BlockEvent`]s and
/// the max retrievable sequence.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct EventsBlocksResponse {
    /// `max_sequence` is the maximum available sequence number to fetch.
    pub max_sequence: i64,
    /// events is an array of [`BlockEvent`]s indicating the order to add and
    /// remove blocks to maintain a canonical view of blockchain state.
    /// Lightweight clients can use this event stream to update state
    /// without implementing their own block syncing logic.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub events: Vec<Option<BlockEvent>>,
}
