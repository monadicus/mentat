use super::*;
use crate::api::Caller;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HealthCheckResponse {
    pub caller: Caller,
    pub msg: String,
    pub node_status: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_status: Option<Value>,
}
