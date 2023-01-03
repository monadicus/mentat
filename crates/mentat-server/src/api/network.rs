//! Houses the traits for the Rosetta Network API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// NetworkAPIServicer defines the api actions for the NetworkAPI service
#[axum::async_trait]
pub trait NetworkApi: Clone + Debug + Default + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

    /// This endpoint returns a list of
    /// [`crate::identifiers::NetworkIdentifier`]s that the Rosetta
    /// server supports.
    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<NetworkListResponse> {
        MentatError::not_implemented()
    }

    /// This endpoint returns the version information and allowed
    /// network-specific types for a [`crate::identifiers::NetworkIdentifier`].
    /// Any [`crate::identifiers::NetworkIdentifier`] returned by
    /// /network/list should be accessible here. Because options are
    /// retrievable in the context of a
    /// [`crate::identifiers::NetworkIdentifier`], it is possible to define
    /// unique options for each network.
    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<NetworkOptionsResponse> {
        MentatError::not_implemented()
    }

    /// This endpoint returns the current status of the network requested. Any
    /// [`crate::identifiers::NetworkIdentifier`] returned by /network/list
    /// should be accessible here.
    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<NetworkStatusResponse> {
        MentatError::not_implemented()
    }
}

crate::router!(NetworkApiRouter, NetworkApi);

impl<Api: NetworkApi> NetworkApiRouter<Api> {
    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/network/list")]
    async fn call_network_list(
        &self,
        caller: Caller,
        data: Option<UncheckedMetadataRequest>,
    ) -> MentatResponse<UncheckedNetworkListResponse> {
        self.asserter.metadata_request(data.as_ref())?;
        let resp = self
            .api
            .network_list(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/network/options")]
    async fn call_network_options(
        &self,
        caller: Caller,
        data: Option<UncheckedNetworkRequest>,
    ) -> MentatResponse<UncheckedNetworkOptionsResponse> {
        self.asserter.network_request(data.as_ref())?;
        let resp = self
            .api
            .network_options(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint only runs in online mode.
    #[tracing::instrument(name = "/network/status")]
    async fn call_network_status(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedNetworkRequest>,
    ) -> MentatResponse<UncheckedNetworkStatusResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.network_request(data.as_ref())?;
            let resp = self
                .api
                .network_status(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}

impl<Api> ToRouter for NetworkApiRouter<Api>
where
    Api: NetworkApi + 'static,
{
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        let list = self.clone();
        let options = self.clone();
        axum::Router::new()
            .route(
                "/list",
                axum::routing::post(
                    |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                     Json(req_data): Json<Option<UncheckedMetadataRequest>>| async move {
                        list.call_network_list(Caller { ip }, req_data).await
                    },
                ),
            )
            .route(
                "/options",
                axum::routing::post(
                    |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                     Json(req_data): Json<Option<UncheckedNetworkRequest>>| async move {
                        options.call_network_options(Caller { ip }, req_data).await
                    },
                ),
            )
            .route(
                "/status",
                axum::routing::post(
                    |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                     State(conf): State<Configuration<CustomConfig>>,
                     Json(req_data): Json<Option<UncheckedNetworkRequest>>| async move {
                        self.call_network_status(Caller { ip }, &conf.mode, req_data)
                            .await
                    },
                ),
            )
    }
}
