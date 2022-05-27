use mentat::{
    api::OptionalApi,
    axum::async_trait,
    conf::Mode,
    errors::Result,
    indexmap::IndexMap,
    responses::NodeConnections,
    serde_json::Value,
    server::RpcCaller,
};

use crate::{request::BitcoinJrpc, responses::Response};
#[derive(Clone, Default)]
pub struct BitcoinOptionalApi;

#[async_trait]
impl OptionalApi for BitcoinOptionalApi {
    async fn node_address(&self, _rpc_caller: &RpcCaller) -> Result<String> {
        Ok(String::new())
    }

    async fn node_connections(
        &self,
        mode: &Mode,
        rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeConnections>> {
        if mode.is_offline() {
            Ok(Some(NodeConnections::Offline))
        } else {
            let result: IndexMap<String, Value> = rpc_caller
                .rpc_call::<Response<IndexMap<String, Value>>>(BitcoinJrpc::new(
                    "getnetworkinfo",
                    &[()],
                ))
                .await?;

            Ok(Some(NodeConnections::Online {
                total: result["connections"].as_u64().unwrap(),
                inbound: result["connections_in"].as_u64().unwrap(),
                outbound: result["connections_out"].as_u64().unwrap(),
            }))
        }
    }

    /* async fn node_net_usage(
        &self,
        mode: &Mode,
        _rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeNetwork>> {
        if mode.is_offline() {
            Ok(Some(NodeNetwork::Offline))
        } else {
        }
    } */
}
