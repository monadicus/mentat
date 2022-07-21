//! The module defines the `Operator` model.

use super::*;

/// [`Operator`] is used by query-related endpoints to determine how to apply
/// conditions.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Operator(pub String);

impl Operator {
    /// If all conditions are satisfied, it is considered a match.
    pub const AND: &'static str = "and";
    /// If any condition is satisfied, it is considered a match.
    pub const OR: &'static str = "or";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::OR | Self::AND => true,
            _ => false,
        }
    }
}
