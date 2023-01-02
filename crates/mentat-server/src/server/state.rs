//! The state for the `Server`.

use std::sync::Arc;

use axum::extract::FromRef;

use crate::conf::*;

/// Defines a state of shared resources for the `axum::Router`.
#[derive(Clone)]
pub struct AppState<CustomConfig: NodeConf> {
    /// The configuration file.
    pub config: Configuration<CustomConfig>,
    /// The node's process id.
    pub node_pid: NodePid,
    /// The server's process id.
    pub server_pid: ServerPid,
}

impl<CustomConfig: NodeConf> FromRef<Arc<AppState<CustomConfig>>> for Configuration<CustomConfig> {
    fn from_ref(state: &Arc<AppState<CustomConfig>>) -> Self {
        state.config.clone()
    }
}

impl<CustomConfig: NodeConf> FromRef<Arc<AppState<CustomConfig>>> for NodePid {
    fn from_ref(state: &Arc<AppState<CustomConfig>>) -> Self {
        state.node_pid
    }
}

impl<CustomConfig: NodeConf> FromRef<Arc<AppState<CustomConfig>>> for ServerPid {
    fn from_ref(state: &Arc<AppState<CustomConfig>>) -> Self {
        state.server_pid
    }
}
