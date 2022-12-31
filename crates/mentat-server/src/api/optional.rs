//! Houses the traits for any additional API endpoints.
//! These traits are easily overridable for custom
//! implementations.

use std::fmt::Debug;

use axum::extract::{ConnectInfo, State};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

use super::*;
use crate::conf::{Configuration, NodePid, ServerPid};

#[axum::async_trait]
/// The `OptionalApi` Trait.
pub trait OptionalApi: Clone + Default + Debug {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

    /// returns local and global chain tips
    async fn synced(&self, _node_caller: &Self::NodeCaller) -> Result<Synced> {
        MentatError::not_implemented()
    }

    /// A default implementation for providing a health check.
    async fn health(
        &self,
        caller: Caller,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
        server_pid: &ServerPid,
        node_pid: &NodePid,
    ) -> Result<HealthCheckResponse> {
        tracing::debug!("health check!");
        let system = System::new_all();
        Ok(HealthCheckResponse {
            caller,
            msg: "Healthy!".to_string(),
            usage: self.usage("server", &system, server_pid.0).await?,
            node: NodeInformation {
                usage: self.usage("node", &system, node_pid.0).await?,
                address: self.node_address(node_caller).await?,
                connections: self.node_connections(mode, node_caller).await?,
                net_usage: self.node_net_usage(mode, node_caller).await?,
            },
            cache_usage: self.check_cache_usage().await?,
        })
    }

    /// A method for getting the usage of a Process.
    async fn usage(&self, process: &str, system: &System, pid: Pid) -> Result<Usage> {
        let proc = system
            .process(pid)
            .merr(|| format!("Could not find `{process}` process pid: `{pid}`."))?;
        let total_cpu_usage = proc.cpu_usage();
        Ok(Usage {
            cpu_usage: (total_cpu_usage / num_cpus::get() as f32),
            total_cpu_usage,
            memory_usage: proc.memory(),
            virtual_memory_usage: proc.virtual_memory(),
            start_time: proc.start_time(),
            run_time: proc.run_time(),
        })
    }

    /// A method for getting the address of the node.
    async fn node_address(&self, _node_caller: &Self::NodeCaller) -> Result<String> {
        Ok(String::new())
    }

    /// A method for getting the number of connections a node has.
    async fn node_connections(
        &self,
        _mode: &Mode,
        _node_caller: &Self::NodeCaller,
    ) -> Result<Option<NodeConnections>> {
        Ok(None)
    }

    /// A method for getting the network usage of a node.
    async fn node_net_usage(
        &self,
        _mode: &Mode,
        _node_caller: &Self::NodeCaller,
    ) -> Result<Option<NodeNetwork>> {
        Ok(None)
    }

    /// A default implementation for providing a cache usage check.
    async fn check_cache_usage(&self) -> Result<Option<Usage>> {
        Ok(None)
    }
}

/// Struct to wrap the `OptionalApi`.
/// This struct helps to define default behavior for running the endpoints
/// on different modes.
#[derive(Clone, Debug, Default)]
pub struct OptionalApiRouter<Api: OptionalApi + Debug> {
    /// api
    pub api: Api,
    /// if health is enabled
    pub enabled: bool,
    /// Caller
    pub node_caller: Api::NodeCaller,
}

impl<Api: OptionalApi + Debug + Send + Sync> OptionalApiRouter<Api> {
    /// For performing a health check on the server.
    #[tracing::instrument(name = "health")]
    pub async fn call_health(
        &self,
        caller: Caller,
        mode: &Mode,
        server_pid: ServerPid,
        node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        if self.enabled {
            Ok(Json(
                self.api
                    .health(caller, mode, &self.node_caller, &server_pid, &node_pid)
                    .await?,
            ))
        } else {
            MentatError::not_implemented()
        }
    }

    /// This endpoint only runs in online mode.
    #[tracing::instrument(name = "health")]
    async fn call_synced(&self, mode: &Mode) -> MentatResponse<Synced> {
        if !self.enabled {
            MentatError::not_implemented()
        } else if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            Ok(Json(self.api.synced(&self.node_caller).await?))
        }
    }
}

impl<Api: OptionalApi + Debug + Send + Sync + 'static> ToRouter for OptionalApiRouter<Api> {
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        let health = self.clone();
        axum::Router::new()
            .route(
                "/health",
                axum::routing::get(
                    |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                     State(conf): State<Configuration<CustomConfig>>,
                     State(server_pid): State<ServerPid>,
                     State(node_pid): State<NodePid>| async move {
                        health
                            .call_health(Caller { ip }, &conf.mode, server_pid, node_pid)
                            .await
                    },
                ),
            )
            .route(
                "/synced",
                axum::routing::get(
                    |State(conf): State<Configuration<CustomConfig>>| async move {
                        self.call_synced(&conf.mode).await
                    },
                ),
            )
    }
}
