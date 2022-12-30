//! Houses the traits for any additional API endpoints.
//! These traits are easily overridable for custom
//! implementations.

use axum::extract::{ConnectInfo, State};
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use tracing::Instrument;

use super::*;
use crate::conf::{Configuration, NodePid, ServerPid};

#[axum::async_trait]
/// The `OptionalApi` Trait.
pub trait OptionalApi: Clone + Default {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Send + Sync + 'static;

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
        server_pid: ServerPid,
        node_pid: NodePid,
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

/// Trait to wrap the `OptionalApi`.
/// This trait helps to define default behavior for running the endpoints
/// on different modes.
#[axum::async_trait]
pub trait OptionalApiRouter: OptionalApi + Clone + Default {
    /// For performing a health check on the server.
    async fn call_health(
        &self,
        _caller: Caller,
        _mode: &Mode,
        _node_caller: &Self::NodeCaller,
        _server_pid: ServerPid,
        _node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        MentatError::not_implemented()
    }

    /// This endpoint only runs in online mode.
    async fn call_synced(
        &self,
        _caller: Caller,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
        _server_pid: ServerPid,
        _node_pid: NodePid,
    ) -> MentatResponse<Synced> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            Ok(Json(self.synced(node_caller).await?))
        }
    }
}

impl<T: OptionalApiRouter + Send + Sync + 'static> ToRouter for T {
    type NodeCaller = T::NodeCaller;

    fn to_router<CustomConfig: NodeConf>(
        &self,
        node_caller: Self::NodeCaller,
    ) -> axum::Router<Arc<AppState<CustomConfig>>> {
        let health_self = self.clone();
        let health_node_caller = node_caller;
        let health = move |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                           State(server_pid): State<ServerPid>,
                           State(node_pid): State<NodePid>,
                           State(conf): State<Configuration<CustomConfig>>| {
            {
                Box::pin(async move {
                    let caller = Caller { ip };
                    tracing::info!("{caller:?}");
                    let resp = health_self
                        .call_health(
                            caller,
                            &conf.mode,
                            &health_node_caller,
                            server_pid,
                            node_pid,
                        )
                        .await;

                    #[cfg(debug_assertions)]
                    tracing::debug!("response /optional/health {resp:?}");
                    resp
                })
            }
            .instrument(tracing::info_span!(stringify!("health")))
        };

        axum::Router::new().route("/health", axum::routing::get(health))
    }
}
