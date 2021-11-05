use super::*;

/// Operator is used by query-related endpoints to determine how to apply conditions.
#[derive(Serialize, Deserialize)]
pub enum Operator {
    /// If any condition is satisfied, it is considered a match.
    #[serde(rename = "or")]
    Or,
    /// If all conditions are satisfied, it is considered a match.
    #[serde(rename = "and")]
    And,
}
