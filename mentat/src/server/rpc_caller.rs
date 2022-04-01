use serde::{de::DeserializeOwned, Serialize};

use crate::conf::{Configuration, NodeConf};

#[derive(Clone, Debug)]
pub struct RpcCaller {
    pub client: reqwest::Client,
    pub node_rpc_url: String,
}

impl RpcCaller {
    pub fn new<CustomConf: NodeConf + DeserializeOwned + Serialize>(
        conf: &Configuration<CustomConf>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            node_rpc_url: NodeConf::build_url(conf),
        }
    }
}
