use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The struct to represent the user who called the endpoint.
pub struct Caller {
    /// The socket address of the user who called the end point.
    pub ip: SocketAddr,
}
