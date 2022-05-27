use mentat::{
    api::OptionalApi,
    axum::async_trait,
    conf::Mode,
    errors::Result,
    responses::{NodeConnections, NodeNetwork},
    server::RpcCaller,
};

#[derive(Clone, Default)]
pub struct SnarkosOptionalApi;

#[async_trait]
impl OptionalApi for SnarkosOptionalApi {
    async fn node_address(&self, _rpc_caller: &RpcCaller) -> Result<String> {
        Ok("unknown".to_string())
    }

    // async fn node_connections(
    //     &self,
    //     mode: &Mode,
    //     _rpc_caller: &RpcCaller,
    // ) -> Result<Option<NodeConnections>> {
    //     if mode.is_offline() {
    // 		Ok(NodeConnections::Offline)
    // 	} else {

    // 	}
    // }

    // async fn node_net_usage(
    //     &self,
    //     mode: &Mode,
    //     _rpc_caller: &RpcCaller,
    // ) -> Result<Option<NodeNetwork>> {
    //     if mode.is_offline() {
    // 		Ok(NodeNetwork::Offline)
    // 	} else {

    // 	}
    // }
}
