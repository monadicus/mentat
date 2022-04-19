//! Defines error type for the Rosetta API.

use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "client")]

/// The Error type for any mentat responses.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg(feature = "client")]
pub struct ApiError {
    /// The http status code.
    pub code: u16,
    /// The message for the error.
    pub message: String,
    /// The optional description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If the method is retriable.
    pub retriable: bool,
    /// Any additional details for the error.
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub details: IndexMap<String, Value>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(desc) = &self.description {
            write!(f, "[{}] {}: {}", self.code, self.message, desc)
        } else {
            write!(f, "[{}] {}", self.code, self.message)
        }
    }
}
