//! The module defines the `Direction` model.

use std::fmt;

use super::*;

/// Used by [`RelatedTransaction`] to indicate the direction of the relation
/// (i.e. cross-shard/cross-network sends may reference backward to an earlier
/// transaction and async execution may reference forward). Can be used to
/// indicate if a transaction relation is from child to parent or the reverse.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Direction(String);

impl Direction {
    /// Direction indicating a transaction relation is from child to parent.
    pub const BACKWARD: &'static str = "backward";
    /// Direction indicating a transaction relation is from parent to child.
    pub const FORWARD: &'static str = "forward";

    /// returns true if the `Direction` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::FORWARD | Self::BACKWARD)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Direction {
    fn from(dir: String) -> Self {
        Self(dir)
    }
}

impl From<&str> for Direction {
    fn from(dir: &str) -> Self {
        dir.to_string().into()
    }
}
