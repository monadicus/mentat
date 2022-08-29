//! The module defines the `SyncStatus`.

use mentat_macros::Nullable;

use super::*;

/// [`SyncStatus`] is used to provide additional context about an
/// implementation's sync status. This object is often used by implementations
/// to indicate healthiness when block data cannot be queried until some sync
/// phase completes or cannot be determined by comparing the timestamp of the
/// most recent block with the current time.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Nullable)]
#[serde(default)]
pub struct NullableSyncStatus {
    /// `CurrentIndex` is the index of the last synced block in the current
    /// stage. This is a separate field from `current_block_identifier` in
    /// [`crate::responses::NetworkStatusResponse`] because blocks with indices
    /// up to and including the `current_index` may not yet be queryable by
    /// the caller. To reiterate, all indices up to and including
    /// `current_block_identifier` in
    /// [`crate::responses::NetworkStatusResponse`] must be queryable via
    /// the `/block` endpoint (excluding indices less than
    /// `oldest_block_identifier`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(option_usize)]
    pub current_index: Option<isize>,
    /// `TargetIndex` is the index of the block that the implementation is
    /// attempting to sync to in the current stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(option_usize)]
    pub target_index: Option<isize>,
    /// Stage is the phase of the sync process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// Synced is a boolean that indicates if an implementation has synced up to
    /// the most recent block. If this field is not populated, the caller should
    /// rely on a traditional tip timestamp comparison to determine if an
    /// implementation is synced. This field is particularly useful for
    /// quiescent blockchains (blocks only produced when there are pending
    /// transactions). In these blockchains, the most recent block could have a
    /// timestamp far behind the current time but the node could be healthy and
    /// at tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
}
