//! The module defines the Direction model.

use super::*;

/// Used by RelatedTransaction to indicate the direction of the relation (i.e.
/// cross-shard/cross-network sends may reference backward to an earlier
/// transaction and async execution may reference forward). Can be used to
/// indicate if a transaction relation is from child to parent or the reverse.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Direction {
    /// Direction indicating a transaction relation is from parent to child.
    #[serde(rename = "forward")]
    Forward,
    /// Direction indicating a transaction relation is from child to parent.
    #[serde(rename = "backward")]
    Backward,
}
