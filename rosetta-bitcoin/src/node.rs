use std::{path::PathBuf, process::Command};

use mentat::{
    async_trait,
    conf::{Configuration, NodeConf},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct NodeConfig {
    data_dir: PathBuf,
    user: String,
    pass: String,
}

#[async_trait]
impl NodeConf for NodeConfig {
    fn node_name() -> String {
        String::from("Bitcoin")
    }

    fn build_url(conf: &Configuration<Self>) -> String {
        format!(
            "{}://{}:{}@{}:{}",
            if conf.secure_http { "https" } else { "http" },
            conf.custom.user,
            conf.custom.pass,
            conf.node_address,
            conf.node_rpc_port
        )
    }

    fn node_command(config: &Configuration<Self>) -> Command {
        let mut command = Command::new(&config.node_path);
        command.args(&[
            // TODO cant bind to address without setting a whitelist
            // &format!("--bind={address}:4132"),
            // &format!("--rpcbind={address}:3032"),
            "-port=4132",
            // TODO `Config options rpcuser and rpcpassword will soon be deprecated.
            // Locally-run instances may remove rpcuser to use cookie-based auth, or may be
            // replaced with rpcauth. Please see share/rpcauth for rpcauth auth generation.`
            &format!("-rpcport={}", config.node_rpc_port),
            &format!("-rpcuser={}", config.custom.user),
            &format!("-rpcpassword={}", config.custom.pass),
            "-txindex=1",
            &format!("--datadir={}", config.custom.data_dir.display()),
        ]);
        command
    }
}
