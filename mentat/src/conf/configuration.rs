use std::{
    fs,
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

use axum::async_trait;
use serde::de::DeserializeOwned;

use super::*;

#[async_trait]
pub trait NodeConf: Default {
    async fn start_node(config: &Configuration<Self>) -> Result<(), Box<dyn std::error::Error>>;

    fn build_url(conf: &Configuration<Self>) -> String {
        format!(
            "{}://{}:{}",
            if conf.secure_http { "https" } else { "http" },
            conf.node_address,
            conf.node_rpc_port
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration<Custom: NodeConf> {
    pub address: Ipv4Addr,
    pub blockchain: String,
    pub mode: Mode,
    pub network: Network,
    pub secure_http: bool,
    pub node_address: String,
    pub node_path: PathBuf,
    pub node_rpc_port: u16,
    pub port: u16,
    pub custom: Custom,
}

impl<Custom> Configuration<Custom>
where
    Custom: NodeConf + DeserializeOwned + Serialize,
{
    pub fn new(custom: Custom) -> Self {
        Self {
            custom,
            ..Default::default()
        }
    }

    pub fn load(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap_or_else(|e| {
            panic!(
                "Failed to read config file at path `{}`: {}",
                path.display(),
                e
            )
        });
        let config: Self = toml::from_str(&content).unwrap_or_else(|e| {
            panic!(
                "Failed to parse config file at path `{}`: {}",
                path.display(),
                e
            )
        });

        if !config.node_path.exists() {
            panic!("Failed to find node at `{}`", config.node_path.display())
        }

        config
    }

    pub fn create_template(path: &Path) {
        fs::create_dir_all(path)
            .unwrap_or_else(|e| panic!("failed to create path `{}`: {}", path.display(), e));

        let default_config = path.join("default.config.toml");
        let content = toml::to_string_pretty(&Self::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to create default toml configuration at `{}`: {}",
                path.display(),
                e
            )
        });

        fs::write(&default_config, content).unwrap_or_else(|e| {
            panic!(
                "failed to write to default config `{}`: {}",
                path.display(),
                e
            )
        });
    }

    pub fn build_url(&self) -> String {
        NodeConf::build_url(self)
    }

    pub async fn start_node(&self) -> Result<(), Box<dyn std::error::Error>> {
        NodeConf::start_node(self).await
    }
}

impl<Custom: NodeConf> Default for Configuration<Custom> {
    fn default() -> Self {
        Self {
            address: Ipv4Addr::new(0, 0, 0, 0),
            blockchain: "UNKNOWN".to_string(),
            mode: Default::default(),
            network: Network::Testnet,
            node_address: "127.0.0.1".to_string(),
            node_path: PathBuf::from("/app/rosetta-mentat-service"),
            node_rpc_port: 4032,
            port: 8080,
            secure_http: true,
            custom: Default::default(),
        }
    }
}
