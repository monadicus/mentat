//! The module defines the `EventsBlocksRequest` request.

use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Clone, Debug, Deserialize, Serialize, Default, Nullable)]
#[serde(default)]
pub struct NullableEventsBlocksRequest {
    /// [`EventsBlocksRequest`] is utilized to fetch a sequence of
    /// [`BlockEvent`]s indicating which blocks were added and removed from
    /// storage to reach the current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// offset is the offset into the event stream to sync events from. If this
    /// field is not populated, we return the limit events backwards from tip.
    /// If this is set to 0, we start from the beginning.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(option_usize)]
    pub offset: Option<isize>,
    /// limit is the maximum number of events to fetch in one call. The
    /// implementation may return "= limit events.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(option_usize)]
    pub limit: Option<isize>,
}
