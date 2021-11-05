use indexmap::IndexMap;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: u32,
    pub message: String,
    pub description: Option<String>,
    pub retriable: bool,
    #[serde(default)]
    pub details: IndexMap<String, Value>
}