use std::{fs, net::Ipv4Addr, path::Path};

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub address: Ipv4Addr,
    pub blockchain: String,
    pub mode: Mode,
    pub network: Network,
    pub node_address: String,
    pub node_port: String,
    pub port: u16,
}

impl Configuration {
    pub fn load(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap_or_else(|e| {
            panic!(
                "failed to read config file at path `{}`: {}",
                path.display(),
                e
            )
        });
        toml::from_str(&content).unwrap_or_else(|e| {
            panic!(
                "failed to parse config file at path `{}`: {}",
                path.display(),
                e
            )
        })
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

impl Default for Configuration {
    fn default() -> Self {
        Self {
            address: Ipv4Addr::new(0, 0, 0, 0),
            blockchain: "UNKNOWN".to_string(),
            mode: Default::default(),
            network: Network::Testnet,
            node_address: "127.0.0.1".to_string(),
            node_port: "4032".to_string(),
            port: 8080,
        }
    }
}
