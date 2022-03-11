use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EventsBlocksRequest {
    /// EventsBlocksRequest is utilized to fetch a sequence of BlockEvents indicating which blocks were added and removed from storage to reach the current state.
    pub network_identifier: NetworkIdentifier,
    /// offset is the offset into the event stream to sync events from. If this field is not populated, we return the limit events backwards from tip. If this is set to 0, we start from the beginning.
    pub offset: Option<u64>,
    /// limit is the maximum number of events to fetch in one call. The implementation may return "= limit events.
    pub limit: Option<u64>,
}
