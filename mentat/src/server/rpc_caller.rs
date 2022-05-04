//! This modules contains the `RpcCaller` for mentat.

use serde::{de::DeserializeOwned, Serialize};

use crate::conf::{Configuration, NodeConf};

/// The `RpcCaller` struct is a wrapper to hold a rpc caller instance
/// that holds a request client and the url for the RPC.
#[derive(Clone, Debug)]
pub struct RpcCaller {
    /// The request client.
    pub client: reqwest::Client,
    /// The RPC url.
    pub node_rpc_url: reqwest::Url,
}

impl RpcCaller {
    /// Creates a new instance of the `RpcCaller`.
    pub fn new<CustomConf: NodeConf + DeserializeOwned + Serialize>(
        conf: &Configuration<CustomConf>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            node_rpc_url: NodeConf::build_url(conf),
        }
    }
}
