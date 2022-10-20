//! The module defines the `HealthCheckResponse` response.

use super::*;

/// The `Usage` struct tracks usage of a Process.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

/// Tracks the number of connections a Node has if it is online mode.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum NodeConnections {
    /// Represents Rosetta offline mode where no outbound connections should
    /// exist.
    Offline,
    /// Represents the number of connections your node has during online mode.
    Online {
        /// The total number of connections.
        total: u64,
        /// The number of inbound connections.
        inbound: u64,
        /// The number of outbound connections.
        outbound: u64,
    },
}

/// Tracks the amount of data sent and received by the node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum NodeNetwork {
    /// Represents Rosetta offline mode where no traffic should be received or
    /// sent.
    Offline,
    /// Represents the amount of data received and sent during online mode.
    Online {
        /// The total number of bytes received.
        bytes_recv: u64,
        /// The total number of bytes sent.
        bytes_sent: u64,
    },
}

/// The node information for a health check operation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeInformation {
    /// The usage of the node.
    pub usage: Usage,
    /// The address of the node.
    pub address: String,
    /// The number of the connections the node has if the operation is
    /// supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<NodeConnections>,
    /// The network usage of the node if the operation is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_usage: Option<NodeNetwork>,
}

/// The `HealthCheckResponse` type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HealthCheckResponse {
    /// Who called the endpoint.
    pub caller: Caller,
    /// The health check message.
    pub msg: String,
    /// The server's usage.
    pub usage: Usage,
    /// The node information.
    pub node: NodeInformation,
    /// The usage of the cache if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_usage: Option<Usage>,
}
