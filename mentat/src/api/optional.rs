//! Houses the traits for any additional API endpoints.
//! These traits are easily overridable for custom
//! implementations.

use axum::async_trait;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

use super::*;
use crate::{conf::NodePid, errors::MapErrMentat};

#[axum::async_trait]
/// The `OptionalApi` Trait.
pub trait OptionalApi: Clone + Default {
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
            node_usage: self.usage("node", &system, node_pid.0).await?,
            node_connections: self.node_connections(mode, rpc_caller).await?,
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

    /// A method for getting the number of connections a node has.
    async fn node_connections(
        &self,
        _mode: &Mode,
        _rpc_caller: RpcCaller,
    ) -> Result<Option<NodeConnections>> {
        Ok(None)
    }

    /// A default implementation for providing a cache usage check.
    async fn check_cache_usage(&self) -> Result<Option<Usage>> {
        Ok(None)
    }
}

#[derive(Clone, Default)]
pub struct UnimplementedOptionalApi;

#[async_trait]
impl OptionalApi for UnimplementedOptionalApi {
    async fn health(
        &self,
        _caller: Caller,
        _mode: &Mode,
        _rpc_caller: RpcCaller,
        _server_pid: Pid,
        _node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        MentatError::not_implemented()
    }
}
