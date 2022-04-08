//! The module defines the `HealthCheckResponse` response.

use super::*;
use crate::api::Caller;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The `HealthCheckResponse` type.
pub struct HealthCheckResponse {
    /// Who called the endpoint.
    pub caller: Caller,
    /// The health check message.
    pub msg: String,
    /// The status of the node.
    pub node_status: Value,
    /// The status of the cache if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_status: Option<Value>,
}
