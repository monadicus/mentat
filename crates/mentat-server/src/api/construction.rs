//! Houses the traits for the Rosetta Construction API.
//! These traits are easily overridable for custom
//! implementations.
use super::*;

/// ConstructionAPIServicer defines the api actions for the ConstructionAPI
/// service
#[axum::async_trait]
pub trait ConstructionApi: Clone + Debug + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

    /// Combine creates a network-specific transaction from an unsigned
    /// transaction and an array of provided signatures. The signed transaction
    /// returned from this method will be sent to the /construction/submit
    /// endpoint by the _caller.
    async fn combine(
        &self,
        _caller: Caller,
        _data: ConstructionCombineRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionCombineResponse> {
        MentatError::not_implemented()
    }

    /// Derive returns the [`crate::identifiers::AccountIdentifier`] associated
    /// with a public key. Constructionchains that require an on-chain action to
    /// create an account should not implement this method.
    async fn derive(
        &self,
        _caller: Caller,
        _data: ConstructionDeriveRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionDeriveResponse> {
        MentatError::not_implemented()
    }

    /// Hash returns the network-specific transaction hash for a
    /// signed transaction.
    async fn hash(
        &self,
        _caller: Caller,
        _data: ConstructionHashRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<TransactionIdentifierResponse> {
        MentatError::not_implemented()
    }

    /// Get any information required to construct a transaction for a specific
    /// network. Meta_data returned here could be a recent hash to use, an
    /// account sequence number, or even arbitrary chain state. The request used
    /// when calling this endpoint is created by calling
    /// /construction/preprocess in an offline environment. You should NEVER
    /// assume that the request sent to this endpoint will be created by the
    /// _caller or populated with any custom parameters. This must occur in
    /// /construction/preprocess. It is important to clarify that this endpoint
    /// should not pre-construct any transactions for the client (this should
    /// happen in /construction/payloads). This endpoint is left purposely
    /// unstructured because of the wide scope of meta_data that could be
    /// required.
    async fn metadata(
        &self,
        _caller: Caller,
        _data: ConstructionMetadataRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionMetadataResponse> {
        MentatError::not_implemented()
    }

    /// Parse is called on both unsigned and signed transactions to understand
    /// the intent of the formulated transaction. This is run as a sanity check
    /// before signing (after /construction/payloads) and before broadcast
    /// (after /construction/combine).
    async fn parse(
        &self,
        _caller: Caller,
        _data: ConstructionParseRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionParseResponse> {
        MentatError::not_implemented()
    }

    /// Payloads is called with an array of operations and the response from
    /// /construction/meta_data. It returns an unsigned transaction blob and a
    /// collection of payloads that must be signed by particular
    /// AccountIdentifiers using a certain [`crate::models::SignatureType`]. The
    /// array of operations provided in transaction construction often times
    /// can not specify all "effects" of a transaction (consider invoked
    /// transactions in Ethereum). However, they can deterministically
    /// specify the "intent" of the transaction, which is sufficient for
    /// construction. For this reason, parsing the corresponding transaction
    /// in the _Data API (when it lands on chain) will contain a superset of
    /// whatever operations were provided during construction.
    async fn payloads(
        &self,
        _caller: Caller,
        _data: ConstructionPayloadsRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionPayloadsResponse> {
        MentatError::not_implemented()
    }

    /// Preprocess is called prior to /construction/payloads to construct a
    /// request for any meta_data that is needed for transaction construction
    /// given (i.e. account nonce). The options object returned from this
    /// endpoint will be sent to the /construction/meta_data endpoint UNMODIFIED
    /// by the _caller (in an offline execution environment). If your
    /// Construction API implementation has configuration options, they MUST be
    /// specified in the /construction/preprocess request (in the meta_data
    /// field).
    async fn preprocess(
        &self,
        _caller: Caller,
        _data: ConstructionPreprocessRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionPreprocessResponse> {
        MentatError::not_implemented()
    }

    /// Submit a pre-signed transaction to the node. This call should not block
    /// on the transaction being included in a block. Rather, it should return
    /// immediately with an indication of whether or not the transaction was
    /// included in the mempool. The transaction submission response should only
    /// return a 200 status if the submitted transaction could be included in
    /// the mempool. Otherwise, it should return an error.
    async fn submit(
        &self,
        _caller: Caller,
        _data: ConstructionSubmitRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<TransactionIdentifierResponse> {
        MentatError::not_implemented()
    }
}

crate::router!(ConstructionApiRouter, ConstructionApi);

impl<Api: ConstructionApi> ConstructionApiRouter<Api> {
    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/combine")]
    async fn call_combine(
        &self,
        caller: Caller,
        data: Option<UncheckedConstructionCombineRequest>,
    ) -> MentatResponse<UncheckedConstructionCombineResponse> {
        self.asserter.construction_combine_request(data.as_ref())?;
        let resp = self
            .api
            .combine(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/derive")]
    async fn call_derive(
        &self,
        caller: Caller,
        data: Option<UncheckedConstructionDeriveRequest>,
    ) -> MentatResponse<UncheckedConstructionDeriveResponse> {
        self.asserter.construction_derive_request(data.as_ref())?;
        let resp = self
            .api
            .derive(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/hash")]
    async fn call_hash(
        &self,
        caller: Caller,
        data: Option<UncheckedConstructionHashRequest>,
    ) -> MentatResponse<UncheckedTransactionIdentifierResponse> {
        self.asserter.construction_hash_request(data.as_ref())?;
        let resp = self
            .api
            .hash(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/metadata")]
    async fn call_metadata(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedConstructionMetadataRequest>,
    ) -> MentatResponse<UncheckedConstructionMetadataResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.construction_metadata_request(data.as_ref())?;
            let resp = self
                .api
                .metadata(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/parse")]
    async fn call_parse(
        &self,
        caller: Caller,
        data: Option<UncheckedConstructionParseRequest>,
    ) -> MentatResponse<UncheckedConstructionParseResponse> {
        self.asserter.construction_parse_request(data.as_ref())?;
        let data: ConstructionParseRequest = data.unwrap().into();
        let resp = self
            .api
            .parse(caller, data, &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/payloads")]
    async fn call_payloads(
        &self,
        caller: Caller,
        data: Option<UncheckedConstructionPayloadsRequest>,
    ) -> MentatResponse<UncheckedConstructionPayloadsResponse> {
        self.asserter.construction_payload_request(data.as_ref())?;
        let resp = self
            .api
            .payloads(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/construction/preprocess")]
    async fn call_preprocess(
        &self,
        caller: Caller,
        data: Option<UncheckedConstructionPreprocessRequest>,
    ) -> MentatResponse<UncheckedConstructionPreprocessResponse> {
        self.asserter
            .construction_preprocess_request(data.as_ref())?;
        let resp = self
            .api
            .preprocess(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint only runs in online mode.
    #[tracing::instrument(name = "/construction/submit")]
    async fn call_submit(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedConstructionSubmitRequest>,
    ) -> MentatResponse<UncheckedTransactionIdentifierResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.construction_submit_request(data.as_ref())?;
            let resp = self
                .api
                .submit(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}

impl<Api> ToRouter for ConstructionApiRouter<Api>
where
    Api: ConstructionApi + 'static,
{
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        let combine = self.clone();
        let derive = self.clone();
        let hash = self.clone();
        let metadata = self.clone();
        let parse = self.clone();
        let payloads = self.clone();
        let preprocess = self.clone();
        axum::Router::new()
        .route(
            "/combine",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedConstructionCombineRequest>>| async move {
                    combine.call_combine(Caller { ip }, req_data).await
                },
            ),
        )
        .route(
            "/derive",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedConstructionDeriveRequest>>| async move {
                    derive.call_derive(Caller { ip }, req_data).await
                },
            ),
        )
        .route(
            "/hash",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedConstructionHashRequest>>| async move {
                    hash.call_hash(Caller { ip }, req_data).await
                },
            ),
        )
        .route(
            "/metadata",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 State(conf): State<Configuration<CustomConfig>>,
                 Json(req_data): Json<Option<UncheckedConstructionMetadataRequest>>| async move {
                    metadata.call_metadata(Caller { ip }, &conf.mode, req_data).await
                },
            ),
        )
        .route(
            "/parse",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedConstructionParseRequest>>| async move {
                    parse.call_parse(Caller { ip }, req_data).await
                },
            ),
        )
        .route(
            "/payloads",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedConstructionPayloadsRequest>>| async move {
                    payloads.call_payloads(Caller { ip }, req_data).await
                },
            ),
        )
        .route(
            "/preprocess",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedConstructionPreprocessRequest>>| async move {
                    preprocess.call_preprocess(Caller { ip }, req_data).await
                },
            ),
        )
        .route(
            "/submit",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 State(conf): State<Configuration<CustomConfig>>,
                 Json(req_data): Json<Option<UncheckedConstructionSubmitRequest>>| async move {
                    self.call_submit(Caller { ip }, &conf.mode, req_data).await
                },
            ),
        )
    }
}
