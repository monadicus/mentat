use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub retriable: bool,
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
