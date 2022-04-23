//! The module defines the `HealthCheckResponse` response.

use super::*;
use crate::Caller;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The `Usage` struct tracks usage of a Process.
pub struct Usage {
    /// Total CPU usage could go over 100% as it includes all cores.
    pub total_cpu_usage: f32,
    /// Single core CPU usage.
    pub cpu_usage: f32,
    /// Memory usage.
    pub memory_usage: u64,
    /// Virtual memory usage.
    pub virtual_memory_usage: u64,
    /// The time started since the epoch in seconds.
    pub start_time: u64,
    /// The time running in seconds.
    pub run_time: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The `HealthCheckResponse` type.
pub struct HealthCheckResponse {
    /// Who called the endpoint.
    pub caller: Caller,
    /// The health check message.
    pub msg: String,
    /// The server's usage.
    pub usage: Usage,
    /// The usage of the node.
    pub node_usage: Usage,
    /// The usage of the cache if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_usage: Option<Usage>,
}
