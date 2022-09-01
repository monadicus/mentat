//! Houses the traits for the Rosetta Construction API.
//! These traits are easily overridable for custom
//! implementations.
use super::*;

/// ConstructionAPIServicer defines the api actions for the ConstructionAPI
/// service
#[axum::async_trait]
pub trait ConstructionApi: Default {
    /// Combine creates a network-specific transaction from an unsigned
    /// transaction and an array of provided signatures. The signed transaction
    /// returned from this method will be sent to the /construction/submit
    /// endpoint by the _caller.
    async fn combine(
        &self,
        _caller: Caller,
        _data: ConstructionCombineRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<ConstructionCombineResponse> {
        MentatError::not_implemented()
    }

    /// Derive returns the [`crate::identifiers::AccountIdentifier`] associated
    /// with a public key. Blockchains that require an on-chain action to
    /// create an account should not implement this method.
    async fn derive(
        &self,
        _caller: Caller,
        _data: ConstructionDeriveRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<ConstructionDeriveResponse> {
        MentatError::not_implemented()
    }

    /// Hash returns the network-specific transaction hash for a
    /// signed transaction.
    async fn hash(
        &self,
        _caller: Caller,
        _data: ConstructionHashRequest,
        _rpc_caller: RpcCaller,
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
        _rpc_caller: RpcCaller,
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
        _rpc_caller: RpcCaller,
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
        _rpc_caller: RpcCaller,
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
        _rpc_caller: RpcCaller,
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
        _rpc_caller: RpcCaller,
    ) -> Result<TransactionIdentifierResponse> {
        MentatError::not_implemented()
    }
}

/// ConstructionAPIRouter defines the required methods for binding the api
/// requests to a responses for the ConstructionAPI The ConstructionAPIRouter
/// implementation should parse necessary information from the http request,
/// pass the data to a ConstructionAPIServicer to perform the required actions,
/// then write the service results to the http response.
#[axum::async_trait]
pub trait ConstructionApiRouter: Clone + ConstructionApi {
    /// This endpoint runs in both offline and online mode.
    async fn call_combine(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionCombineRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedConstructionCombineResponse> {
        asserter.construction_combine_request(data.as_ref())?;
        let resp = self
            .combine(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_derive(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionDeriveRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedConstructionDeriveResponse> {
        asserter.construction_derive_request(data.as_ref())?;
        let resp = self
            .derive(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_hash(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionHashRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedTransactionIdentifierResponse> {
        asserter.construction_hash_request(data.as_ref())?;
        let resp = self
            .hash(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_metadata(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionMetadataRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedConstructionMetadataResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.construction_metadata_request(data.as_ref())?;
            let resp = self
                .metadata(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_parse(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionParseRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedConstructionParseResponse> {
        asserter.construction_parse_request(data.as_ref())?;
        let data: ConstructionParseRequest = data.unwrap().into();
        let resp = self.parse(caller, data, rpc_caller).await?.into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_payloads(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionPayloadsRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedConstructionPayloadsResponse> {
        asserter.construction_payload_request(data.as_ref())?;
        let resp = self
            .payloads(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_preprocess(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionPreprocessRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedConstructionPreprocessResponse> {
        asserter.construction_preprocess_request(data.as_ref())?;
        let resp = self
            .preprocess(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        Ok(Json(resp))
    }

    /// This endpoint only runs in online mode.
    async fn call_submit(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedConstructionSubmitRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedTransactionIdentifierResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.construction_submit_request(data.as_ref())?;
            let resp = self
                .submit(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}
