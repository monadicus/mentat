use std::process::Command;

use mentat::{
    async_trait,
    conf::{Configuration, NodeConf},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct NodeConfig;

#[async_trait]
impl NodeConf for NodeConfig {
    fn node_name() -> String {
        String::from("SnarkOS")
    }

    fn node_command(config: &Configuration<Self>) -> Command {
        // TODO: make it so snarkos checks for updates and rebuilds automatically.
        let mut command = Command::new(&config.node_path);
        command.args(&[
            "--node",
            &format!("{}:4132", config.address),
            "--rpc",
            &format!("{}:{}", config.address, config.node_rpc_port),
            "--trial",
            "--verbosity",
            "2",
        ]);
        command
    }
}
