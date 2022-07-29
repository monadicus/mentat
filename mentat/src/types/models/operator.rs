//! The module defines the `Operator` model.

use std::fmt;

use super::*;

/// [`Operator`] is used by query-related endpoints to determine how to apply
/// conditions.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NullableOperator(String);

impl NullableOperator {
    /// If all conditions are satisfied, it is considered a match.
    pub const AND: &'static str = "and";
    /// If any condition is satisfied, it is considered a match.
    pub const OR: &'static str = "or";

    /// returns true if the `Operator` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::OR | Self::AND)
    }
}

impl fmt::Display for NullableOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NullableOperator {
    fn from(op: String) -> Self {
        Self(op)
    }
}

impl From<&str> for NullableOperator {
    fn from(op: &str) -> Self {
        op.to_string().into()
    }
}

/// [`Operator`] is used by query-related endpoints to determine how to apply
/// conditions.
#[derive(Default, Debug, Clone)]
pub enum Operator {
    #[default]
    /// If all conditions are satisfied, it is considered a match.
    And,
    /// If any condition is satisfied, it is considered a match.
    Or,
}

impl From<NullableOperator> for Operator {
    fn from(other: NullableOperator) -> Self {
        match other.0.as_ref() {
            NullableOperator::AND => Self::And,
            NullableOperator::OR => Self::Or,
            i => panic!("unsupported Operator: {i}"),
        }
    }
}

impl From<Operator> for NullableOperator {
    fn from(other: Operator) -> Self {
        match other {
            Operator::And => Self::AND.into(),
            Operator::Or => Self::OR.into(),
        }
    }
}
