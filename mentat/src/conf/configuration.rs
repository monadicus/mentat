use std::{fs, path::Path};

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub blockchain: String,
    pub mode: Mode,
    pub network: Network,
    pub port: u16,
}

impl Configuration {
    pub fn load(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }

    pub fn create_template(path: &Path) {
        fs::create_dir_all(path).unwrap();

        let template = path.join("_template.toml");
        let content = toml::to_string_pretty(&Self::default()).unwrap();

        fs::write(&template, content).unwrap();
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            blockchain: "UNKNOWN".to_string(),
            mode: Default::default(),
            network: Network::Testnet,
            port: 8080,
        }
    }
}
