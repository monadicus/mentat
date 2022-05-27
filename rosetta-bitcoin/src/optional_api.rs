use mentat::{
    api::OptionalApi,
    axum::async_trait,
    conf::Mode,
    errors::Result,
    responses::{NodeConnections, NodeNetwork},
    server::RpcCaller,
};

use crate::{request::BitcoinJrpc, responses::*};
#[derive(Clone, Default)]
pub struct BitcoinOptionalApi;

#[async_trait]
impl OptionalApi for BitcoinOptionalApi {
    async fn node_address(&self, rpc_caller: &RpcCaller) -> Result<String> {
        let result = rpc_caller
            .rpc_call::<Response<Address>>(BitcoinJrpc::new("getnodeaddresses ", &[()]))
            .await?;

        Ok(result.address)
    }

    async fn node_connections(
        &self,
        mode: &Mode,
        rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeConnections>> {
        if mode.is_offline() {
            Ok(Some(NodeConnections::Offline))
        } else {
            let result = rpc_caller
                .rpc_call::<Response<Connections>>(BitcoinJrpc::new("getnetworkinfo", &[()]))
                .await?;

            Ok(Some(NodeConnections::Online {
                total: result.connections,
                inbound: result.connections_in,
                outbound: result.connections_out,
            }))
        }
    }

    async fn node_net_usage(
        &self,
        mode: &Mode,
        rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeNetwork>> {
        if mode.is_offline() {
            Ok(Some(NodeNetwork::Offline))
        } else {
            let result = rpc_caller
                .rpc_call::<Response<Network>>(BitcoinJrpc::new("getnettotals", &[()]))
                .await?;

            Ok(Some(NodeNetwork::Online {
                bytes_recv: result.totalbytesrecv,
                bytes_sent: result.totalbytessent,
            }))
        }
    }
}
