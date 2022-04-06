//! This module defines the configuration settings used to start and interact
//! with a node instance.

use std::{
    fs,
    io::{BufRead, BufReader, Read},
    net::Ipv4Addr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use axum::async_trait;
use serde::de::DeserializeOwned;

use super::*;

///
/// Custom configuration settings for running a node.\
/// Any fields specified here will be included in [`Configuration`] and listed
/// as configurable fields in the config file that the user provides.
#[async_trait]
pub trait NodeConf: Clone + Default + Send + Serialize + Sync + 'static {
    /// The name of the blockchain run by the node.
    const BLOCKCHAIN: &'static str;

    // TODO: replace with bitcoin example once bitcoin is containerized
    ///
    /// The user specified command for running a node.
    ///
    /// ```no_run
    /// fn node_command(config: &Configuration<Self>) -> Command {
    ///     let mut command = Command::new(&config.node_path);
    ///     command.args(&[
    ///         "--node",
    ///         &format!("{}:4132", config.address),
    ///         "--rpc",
    ///         &format!("{}:{}", config.address, config.node_rpc_port),
    ///         "--trial",
    ///         "--verbosity",
    ///         "2",
    ///     ]);
    ///     command
    /// }
    /// ```
    fn node_command(config: &Configuration<Self>) -> Command;

    ///
    /// Makes a system call with the command returned by
    /// [`NodeConf::node_command`] to spawn the node. The default
    /// implementation should be fine in most cases.\ The user can change
    /// `NodeConf::log` to control how the node output is logged in the
    /// terminal.
    fn start_node(config: &Configuration<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = Self::node_command(config)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        Self::log(stdout, false);
        Self::log(stderr, true);
        Ok(())
    }

    ///
    /// Used to control how the node logs its output to the console.\
    /// The default implementation uses the [`tracing`] crate to print `stdout`
    /// and `stderr` to console.
    fn log<T: 'static + Read + Send>(out: T, err: bool) {
        let mut reader = BufReader::new(out).lines();
        thread::spawn(move || {
            while let Some(Ok(line)) = reader.next() {
                if err {
                    tracing::error!("{}: {line}", Self::BLOCKCHAIN);
                } else {
                    tracing::info!("{}: {line}", Self::BLOCKCHAIN);
                }
            }
        });
    }

    ///
    /// Builds the url used to call the node using the settings in the user
    /// config. The default implementation may need to be changed if a
    /// custom url format is needed.
    fn build_url(conf: &Configuration<Self>) -> String {
        format!(
            "{}://{}:{}",
            if conf.secure_http { "https" } else { "http" },
            conf.node_address,
            conf.node_rpc_port
        )
    }
}

///
/// The user specified configuration settings for a node.
/// Has an extra field called `custom` that can contain any configuration
/// settings specific to the rosetta implementation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration<Custom: NodeConf> {
    /// The Ipv4 that rosetta will run from
    pub address: Ipv4Addr,
    /// The port to bind rosetta to.
    pub port: u16,
    /// The network mode to run rosetta in. Accepts `online` and `offline`.
    pub mode: Mode,
    /// The path to the node binary.
    pub node_path: PathBuf,
    /// The network to run the node on. Defaults to `mainnet`.
    pub network: Network,
    /// If `https` is preferred.
    pub secure_http: bool,
    /// The Ipv4 that the node will run from.
    pub node_address: Ipv4Addr,
    /// The port that the node will bind to.
    pub node_rpc_port: u16,
    /// Configuration settings specific to the rosetta implementation
    #[serde(default)]
    pub custom: Custom,
}

impl<Custom> Configuration<Custom>
where
    Custom: DeserializeOwned + NodeConf,
{
    ///
    /// Loads a configuration file from the supplied path.
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

    ///
    /// Generates a configuration file and writes it to the supplied path.
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

impl<Custom: NodeConf> Default for Configuration<Custom> {
    fn default() -> Self {
        Self {
            address: Ipv4Addr::new(0, 0, 0, 0),
            mode: Default::default(),
            network: Network::Testnet,
            node_address: Ipv4Addr::new(0, 0, 0, 0),
            node_path: PathBuf::from("/app/rosetta-mentat-service"),
            node_rpc_port: 4032,
            port: 8080,
            secure_http: true,
            custom: Default::default(),
        }
    }
}
