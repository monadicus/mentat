//! The module defines the `Operator` model.

use std::fmt;

use super::*;

/// [`Operator`] is used by query-related endpoints to determine how to apply
/// conditions.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Operator(String);

impl Operator {
    /// If all conditions are satisfied, it is considered a match.
    pub const AND: &'static str = "and";
    /// If any condition is satisfied, it is considered a match.
    pub const OR: &'static str = "or";

    /// returns true if the `Operator` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::OR | Self::AND)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Operator {
    fn from(op: String) -> Self {
        Self(op)
    }
}

impl From<&str> for Operator {
    fn from(op: &str) -> Self {
        op.to_string().into()
    }
}
