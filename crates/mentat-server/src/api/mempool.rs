//! Houses the traits for the Rosetta Mempool API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// MempoolAPIServicer defines the api actions for the MempoolAPI service
#[axum::async_trait]
pub trait MempoolApi: Clone + Debug + Default + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

    /// Get all [`crate::identifiers::TransactionIdentifier`]s in the mempool
    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<MempoolResponse> {
        MentatError::not_implemented()
    }

    /// Get a transaction in the mempool by its
    /// [`crate::identifiers::TransactionIdentifier`]. This is a separate
    /// request than fetching a block transaction (/block/transaction)
    /// because some blockchain nodes need to know that a transaction query
    /// is for something in the mempool instead of a transaction in a block.
    /// Transactions may not be fully parsable until they are in a block
    /// (ex: may not be possible to determine the fee to pay
    /// before a transaction is executed). On this endpoint, it is ok that
    /// returned transactions are only estimates of what may actually be
    /// included in a block.
    async fn mempool_transaction(
        &self,
        _caller: Caller,
        _data: MempoolTransactionRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<MempoolTransactionResponse> {
        MentatError::not_implemented()
    }
}

crate::router!(MempoolApiRouter, MempoolApi);

impl<Api: MempoolApi> MempoolApiRouter<Api> {
    /// This endpoint only runs in online mode.
    #[tracing::instrument(name = "/mempool")]
    async fn call_mempool(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedNetworkRequest>,
    ) -> MentatResponse<UncheckedMempoolResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.network_request(data.as_ref())?;
            let resp: UncheckedMempoolResponse = self
                .api
                .mempool(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    #[tracing::instrument(name = "/mempool/transaction")]
    async fn call_mempool_transaction(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedMempoolTransactionRequest>,
    ) -> MentatResponse<UncheckedMempoolTransactionResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.mempool_transaction_request(data.as_ref())?;
            let resp: UncheckedMempoolTransactionResponse = self
                .api
                .mempool_transaction(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}

impl<Api> ToRouter for MempoolApiRouter<Api>
where
    Api: MempoolApi + 'static,
{
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        let mempool = self.clone();
        axum::Router::new()
        .route(
            "/",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 State(conf): State<Configuration<CustomConfig>>,
                 Json(req_data): Json<Option<UncheckedNetworkRequest>>| async move {
                    mempool.call_mempool(Caller { ip }, &conf.mode, req_data).await
                },
            ),
        )
        .route(
            "/transaction",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 State(conf): State<Configuration<CustomConfig>>,
                 Json(req_data): Json<Option<UncheckedMempoolTransactionRequest>>| async move {
                    self.call_mempool_transaction(Caller { ip }, &conf.mode, req_data).await
                },
            ),
        )
    }
}
