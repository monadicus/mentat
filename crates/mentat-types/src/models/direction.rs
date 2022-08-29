//! The module defines the `Direction` model.

use std::fmt;

use super::*;

/// Used by [`RelatedTransaction`] to indicate the direction of the relation
/// (i.e. cross-shard/cross-network sends may reference backward to an earlier
/// transaction and async execution may reference forward). Can be used to
/// indicate if a transaction relation is from child to parent or the reverse.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UncheckedDirection(String);

impl UncheckedDirection {
    /// Direction indicating a transaction relation is from child to parent.
    pub const BACKWARD: &'static str = "backward";
    /// Direction indicating a transaction relation is from parent to child.
    pub const FORWARD: &'static str = "forward";

    /// returns true if the `Direction` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::FORWARD | Self::BACKWARD)
    }

    /// returns true if the underlying string is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for UncheckedDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UncheckedDirection {
    fn from(dir: String) -> Self {
        Self(dir)
    }
}

impl From<&str> for UncheckedDirection {
    fn from(dir: &str) -> Self {
        dir.to_string().into()
    }
}

/// Used by [`RelatedTransaction`] to indicate the direction of the relation
/// (i.e. cross-shard/cross-network sends may reference backward to an earlier
/// transaction and async execution may reference forward). Can be used to
/// indicate if a transaction relation is from child to parent or the reverse.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    /// Direction indicating a transaction relation is from child to parent.
    #[default]
    Backward,
    /// Direction indicating a transaction relation is from parent to child.
    Forward,
}

impl From<UncheckedDirection> for Direction {
    fn from(other: UncheckedDirection) -> Self {
        match other.0.as_ref() {
            UncheckedDirection::BACKWARD => Self::Backward,
            UncheckedDirection::FORWARD => Self::Forward,
            i => panic!("unsupported Direction: {i}"),
        }
    }
}

impl From<Direction> for UncheckedDirection {
    fn from(other: Direction) -> Self {
        match other {
            Direction::Backward => Self::BACKWARD.into(),
            Direction::Forward => Self::FORWARD.into(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Backward => write!(f, "backward"),
            Direction::Forward => write!(f, "forward"),
        }
    }
}
