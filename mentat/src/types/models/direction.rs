//! The module defines the `Direction` model.

use super::*;

/// Used by [`RelatedTransaction`] to indicate the direction of the relation
/// (i.e. cross-shard/cross-network sends may reference backward to an earlier
/// transaction and async execution may reference forward). Can be used to
/// indicate if a transaction relation is from child to parent or the reverse.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Direction(pub String);

impl Direction {
    /// Direction indicating a transaction relation is from child to parent.
    pub const BACKWARD: &'static str = "backward";
    /// Direction indicating a transaction relation is from parent to child.
    pub const FORWARD: &'static str = "forward";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::FORWARD | Self::BACKWARD => true,
            _ => false,
        }
    }
}
