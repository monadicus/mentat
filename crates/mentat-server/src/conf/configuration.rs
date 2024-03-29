//! Defines the configuration settings used to start and interact
//! with a node instance.

use std::{
    fs,
    io::{BufRead, BufReader, Read},
    net::Ipv4Addr,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
    thread,
};

use axum::async_trait;
use serde::de::DeserializeOwned;
use sysinfo::{Pid, PidExt};

use super::*;

/// A wrapper type around a Pid.
/// So we can write our functionality around it.
#[derive(Clone, Copy, Debug)]
pub struct NodePid(pub Pid);

/// A wrapper type around a Pid.
/// So we can write our functionality around it.
#[derive(Clone, Copy, Debug)]
pub struct ServerPid(pub Pid);

/// Custom configuration settings for running a node.
///
/// Any fields specified here will be included in [`Configuration`] and listed
/// as configurable fields in the config file that the user provides.
#[async_trait]
pub trait NodeConf: Clone + Default + Send + Serialize + Sync + 'static {
    /// The name of the blockchain run by the node.
    const BLOCKCHAIN: &'static str;

    /// The command for loading the node `Configuration`.
    ///
    /// WARNING: This defaults to assuming that the first argument passed to the
    /// process contains a path to the config file. Therefor this function
    /// should absolutely be overridden if you are using your own argument
    /// parsing.
    fn load_config() -> Configuration<Self>
    where
        Self: DeserializeOwned,
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("Expected usage: <{}> <configuration file>", args[0]);
            std::process::exit(1);
        }

        let path = std::path::PathBuf::from(&args[1]);
        Configuration::load(&path)
    }

    /// The user specified command for running a node.
    fn node_command(config: &Configuration<Self>) -> Command;

    /// Makes a system call with the command returned by
    /// `NodeConf::node_command` to spawn the node. The default
    /// implementation should be fine in most cases.
    ///
    /// The user can change `NodeConf::log` to control how the node output is
    /// logged in the terminal.
    fn start_node(config: &Configuration<Self>) -> NodePid {
        let mut child = Self::node_command(config)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to start node: `{e}`"));
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        Self::log(stdout, false);
        Self::log(stderr, true);

        NodePid(Pid::from_u32(child.id()))
    }

    /// Used to control how the node logs its output to the console.
    ///
    /// The default implementation uses the tracing crate to print `stdout`
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
}

/// The user specified configuration settings for a node.
/// Has an extra field called `custom` that can contain any configuration
/// settings specific to the rosetta implementation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration<Custom: NodeConf> {
    /// The Ipv4 that rosetta will run from
    pub address: Ipv4Addr,
    /// The port to bind rosetta to.
    pub port: u16,
    /// The network mode to run rosetta in. Accepts `Online` and `Offline`.
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
    #[serde(
        default,
        skip_serializing_if = "Configuration::<Custom>::skip_serializing_custom"
    )]
    pub custom: Custom,
}

impl<Custom: NodeConf> AsRef<Configuration<Custom>> for Configuration<Custom> {
    fn as_ref(&self) -> &Configuration<Custom> {
        self
    }
}

impl<Custom> Configuration<Custom>
where
    Custom: NodeConf,
{
    /// skips serializing the custom struct if it contains no fields
    fn skip_serializing_custom(_: &Custom) -> bool {
        std::mem::size_of::<Custom>() == 0
    }

    /// Loads a configuration file from the supplied path.
    pub fn load(path: &Path) -> Self
    where
        Custom: DeserializeOwned,
    {
        let path = path.with_extension("toml");

        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap_or_else(|e| {
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
        } else {
            Self::create_template(&path);
            println!("created config file `{}`", path.display());
            exit(1);
        }
    }

    /// Generates a configuration file and writes it to the supplied path.
    pub fn create_template(path: &Path) {
        if let Some(p) = path.parent() {
            fs::create_dir_all(p)
                .unwrap_or_else(|e| panic!("failed to create path `{}`: {}", p.display(), e));
        }

        let content = toml::to_string_pretty(&Self::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to create default toml configuration at `{}`: {}",
                path.display(),
                e
            )
        });

        fs::write(path, content).unwrap_or_else(|e| {
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
        let custom = <Custom>::default();
        Self {
            address: Ipv4Addr::new(0, 0, 0, 0),
            mode: Default::default(),
            network: Network::Testnet,
            node_address: Ipv4Addr::new(0, 0, 0, 0),
            node_path: PathBuf::from("/app/rosetta-mentat-service"),
            node_rpc_port: 4032,
            port: 8080,
            secure_http: true,
            custom,
        }
    }
}
