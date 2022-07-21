//! The module defines the `Operator` model.

use super::*;

/// [`Operator`] is used by query-related endpoints to determine how to apply
/// conditions.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Operator {
    /// If any condition is satisfied, it is considered a match.
    #[serde(rename = "or")]
    Or,
    /// If all conditions are satisfied, it is considered a match.
    #[serde(rename = "and")]
    And,
}
