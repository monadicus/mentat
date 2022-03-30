use super::serve_exports::Configuration;

#[derive(Clone, Debug)]
pub struct RpcCaller {
    pub client: reqwest::Client,
    pub node_rpc_url: String,
}

impl RpcCaller {
    pub fn new<CustomConf>(conf: &Configuration<CustomConf>) -> Self
    where
        CustomConf: Default,
    {
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
