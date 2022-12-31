//! Houses the traits for the Rosetta Call API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// CallAPIServicer defines the api actions for the CallAPI service
#[axum::async_trait]
pub trait CallApi: Clone + Debug + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

    /// Make a Network-Specific Procedure Call
    async fn call(
        &self,
        _caller: Caller,
        _data: CallRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<CallResponse> {
        MentatError::not_implemented()
    }
}

crate::router!(CallApiRouter, CallApi);

impl<Api: CallApi> CallApiRouter<Api> {
    /// This endpoint only runs in online mode
    #[tracing::instrument(name = "/call")]
    async fn call_call(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedCallRequest>,
    ) -> MentatResponse<UncheckedCallResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.call_request(data.as_ref())?;
            Ok(Json(
                self.api
                    .call(caller, data.unwrap().into(), &self.node_caller)
                    .await?
                    .into(),
            ))
        }
    }
}

impl<Api> ToRouter for CallApiRouter<Api>
where
    Api: CallApi + 'static,
{
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        axum::Router::new().route(
            "/",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 State(conf): State<Configuration<CustomConfig>>,
                 Json(req_data): Json<Option<UncheckedCallRequest>>| async move {
                    self.call_call(Caller { ip }, &conf.mode, req_data).await
                },
            ),
        )
    }
}
