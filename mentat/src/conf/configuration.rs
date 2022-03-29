use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub mode: Mode,
    pub network: Network,
    pub port: u16,
}
