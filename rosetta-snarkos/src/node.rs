use std::process::{Child, Command, Stdio};

use mentat::{
    async_trait,
    conf::{Configuration, NodeConf},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct NodeConfig;

#[async_trait]
impl NodeConf for NodeConfig {
    fn node_name() -> String {
        String::from("SnarkOS")
    }

    async fn start_node(config: &Configuration<Self>) -> Result<Child, Box<dyn std::error::Error>> {
        // TODO: make it so snarkos checks for updates and rebuilds automatically.
        Ok(Command::new(&config.node_path)
            .args(&[
                "--node",
                &format!("{}:4132", config.address),
                "--rpc",
                &format!("{}:{}", config.address, config.node_rpc_port),
                "--trial",
                "--verbosity",
                "2",
            ])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?)
    }
}
