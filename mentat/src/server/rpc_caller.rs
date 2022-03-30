use super::serve_exports::Configuration;

#[derive(Clone, Debug)]
pub struct RpcCaller {
    pub client: reqwest::Client,
    pub node_rpc_url: String,
}

impl RpcCaller {
    pub fn new(conf: &Configuration) -> Self {
        Self {
            client: reqwest::Client::new(),
            node_rpc_url: format!(
                "{}://{}:{}",
                if conf.secure_http { "https" } else { "http" },
                conf.node_address,
                conf.node_rpc_port
            ),
        }
    }
}
