//! Houses the traits for any additional API endpoints.
//! These traits are easily overridable for custom
//! implementations.

use sysinfo::{Pid, ProcessExt, System, SystemExt};

use super::*;
use crate::conf::NodePid;

#[axum::async_trait]
/// The `OptionalApi` Trait.
pub trait OptionalApi: Clone + Default {
    /// returns local and global chain tips
    async fn synced(&self, _rpc_caller: RpcCaller) -> MentatResponse<Synced> {
        MentatError::not_implemented()
    }

    /// A default implementation for providing a health check.
    async fn health(
        &self,
        caller: Caller,
        mode: &Mode,
        rpc_caller: RpcCaller,
        server_pid: Pid,
        node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        tracing::debug!("health check!");
        let system = System::new_all();
        Ok(Json(HealthCheckResponse {
            caller,
            msg: "Healthy!".to_string(),
            usage: self.usage("server", &system, server_pid).await?,
            node: NodeInformation {
                usage: self.usage("node", &system, node_pid.0).await?,
                address: self.node_address(&rpc_caller).await?,
                connections: self.node_connections(mode, &rpc_caller).await?,
                net_usage: self.node_net_usage(mode, &rpc_caller).await?,
            },
            cache_usage: self.check_cache_usage().await?,
        }))
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
    async fn node_address(&self, _rpc_caller: &RpcCaller) -> Result<String> {
        Ok(String::new())
    }

    /// A method for getting the number of connections a node has.
    async fn node_connections(
        &self,
        _mode: &Mode,
        _rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeConnections>> {
        Ok(None)
    }

    /// A method for getting the network usage of a node.
    async fn node_net_usage(
        &self,
        _mode: &Mode,
        _rpc_caller: &RpcCaller,
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
pub trait CallerOptionalApi: OptionalApi + Clone + Default {
    /// For performing a health check on the server.
    async fn call_health(
        &self,
        _caller: Caller,
        _mode: &Mode,
        _rpc_caller: RpcCaller,
        _server_pid: Pid,
        _node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        MentatError::not_implemented()
    }

    /// This endpoint only runs in online mode.
    async fn call_synced(
        &self,
        _caller: Caller,
        mode: &Mode,
        rpc_caller: RpcCaller,
        _server_pid: Pid,
        _node_pid: NodePid,
    ) -> MentatResponse<Synced> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            self.synced(rpc_caller).await
        }
    }
}
