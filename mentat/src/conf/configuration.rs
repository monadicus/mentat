use std::{
    fs,
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration<Custom>
where
    Custom: Default,
{
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
    Custom: Default + DeserializeOwned + Serialize,
{
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
}

impl<Custom: Default> Default for Configuration<Custom>
where
    Custom: Default,
{
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
