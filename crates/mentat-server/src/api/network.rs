//! Houses the traits for the Rosetta Network API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// NetworkAPIServicer defines the api actions for the NetworkAPI service
#[axum::async_trait]
pub trait NetworkApi {
    /// the caller used to interact with the underlying node
    type NodeCaller: Send + Sync;

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

/// NetworkAPIRouter defines the required methods for binding the api requests
/// to a responses for the NetworkAPI
/// The NetworkAPIRouter implementation should parse necessary information from
/// the http request, pass the data to a NetworkAPIServicer to perform the
/// required actions, then write the service results to the http response.
#[axum::async_trait]
pub trait NetworkApiRouter: NetworkApi + Clone + Default {
    /// This endpoint runs in both offline and online mode.
    async fn call_network_list(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedMetadataRequest>,
        _mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedNetworkListResponse> {
        asserter.metadata_request(data.as_ref())?;
        let resp = self
            .network_list(caller, data.unwrap().into(), node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_network_options(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedNetworkRequest>,
        _mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedNetworkOptionsResponse> {
        asserter.network_request(data.as_ref())?;
        let resp = self
            .network_options(caller, data.unwrap().into(), node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint only runs in online mode.
    async fn call_network_status(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedNetworkRequest>,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedNetworkStatusResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.network_request(data.as_ref())?;
            let resp = self
                .network_status(caller, data.unwrap().into(), node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}
