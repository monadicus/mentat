//! The module defines the OperationStatus.

use super::*;

/// Struct for the `Operation` Status.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct OperationStatus {
    /// The status of the operation.
    pub status: String,
    /// The success of the operation.
    pub successful: bool,
}
