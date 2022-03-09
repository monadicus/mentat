use super::*;

#[async_trait::async_trait]
pub trait ConstructionApi: Send + Sync {
    /// Combine creates a network-specific transaction from an unsigned transaction and an array of provided signatures. The signed transaction returned from this method will be sent to the /construction/submit endpoint by the _caller.
    async fn combine(
        &self,
        _caller: Caller,
        _data: ConstructionCombineRequest,
    ) -> Response<ConstructionCombineResponse> {
        ApiError::not_implemented()
    }

    /// Derive returns the AccountIdentifier associated with a public key. Blockchains that require an on-chain action to create an account should not implement this method.
    async fn derive(
        &self,
        _caller: Caller,
        _data: ConstructionDeriveRequest,
    ) -> Response<ConstructionDeriveResponse> {
        ApiError::not_implemented()
    }

    /// TransactionHash returns the network-specific transaction hash for a signed transaction.
    async fn hash(
        &self,
        _caller: Caller,
        _data: ConstructionHashRequest,
    ) -> Response<TransactionIdentifierResponse> {
        ApiError::not_implemented()
    }

    /// Get any information required to construct a transaction for a specific network. Meta_data returned here could be a recent hash to use, an account sequence number, or even arbitrary chain state. The request used when calling this endpoint is created by calling /construction/preprocess in an offline environment. You should NEVER assume that the request sent to this endpoint will be created by the _caller or populated with any custom parameters. This must occur in /construction/preprocess. It is important to clarify that this endpoint should not pre-construct any transactions for the client (this should happen in /construction/payloads). This endpoint is left purposely unstructured because of the wide scope of meta_data that could be required.
    async fn metadata(
        &self,
        _caller: Caller,
        _data: ConstructionMetadataRequest,
    ) -> Response<ConstructionMetadataResponse> {
        ApiError::not_implemented()
    }

    /// Parse is called on both unsigned and signed transactions to understand the intent of the formulated transaction. This is run as a sanity check before signing (after /construction/payloads) and before broadcast (after /construction/combine).
    async fn parse(
        &self,
        _caller: Caller,
        _data: ConstructionParseRequest,
    ) -> Response<ConstructionParseResponse> {
        ApiError::not_implemented()
    }

    /// Payloads is called with an array of operations and the response from /construction/meta_data. It returns an unsigned transaction blob and a collection of payloads that must be signed by particular AccountIdentifiers using a certain SignatureType. The array of operations provided in transaction construction often times can not specify all "effects" of a transaction (consider invoked transactions in Ethereum). However, they can deterministically specify the "intent" of the transaction, which is sufficient for construction. For this reason, parsing the corresponding transaction in the _Data API (when it lands on chain) will contain a superset of whatever operations were provided during construction.
    async fn payloads(
        &self,
        _caller: Caller,
        _data: ConstructionPayloadsRequest,
    ) -> Response<ConstructionPayloadsResponse> {
        ApiError::not_implemented()
    }

    /// Preprocess is called prior to /construction/payloads to construct a request for any meta_data that is needed for transaction construction given (i.e. account nonce). The options object returned from this endpoint will be sent to the /construction/meta_data endpoint UNMODIFIED by the _caller (in an offline execution environment). If your Construction API implementation has configuration options, they MUST be specified in the /construction/preprocess request (in the meta_data field).
    async fn preprocess(
        &self,
        _caller: Caller,
        _data: ConstructionPreprocessRequest,
    ) -> Response<ConstructionPreprocessResponse> {
        ApiError::not_implemented()
    }

    /// Submit a pre-signed transaction to the node. This call should not block on the transaction being included in a block. Rather, it should return immediately with an indication of whether or not the transaction was included in the mempool. The transaction submission response should only return a 200 status if the submitted transaction could be included in the mempool. Otherwise, it should return an error.
    async fn submit(
        &self,
        _caller: Caller,
        _data: ConstructionSubmitRequest,
    ) -> Response<TransactionIdentifierResponse> {
        ApiError::not_implemented()
    }
}

#[async_trait::async_trait]
pub trait CallConstructionApi: ConstructionApi + Send + Sync {
    async fn call_combine(
        &self,
        caller: Caller,
        data: ConstructionCombineRequest,
        mode: &ModeState,
    ) -> Response<ConstructionCombineResponse> {
        if mode.is_online() {
            ApiError::wrong_network(&data)
        } else {
            self.combine(caller, data).await
        }
    }

    async fn call_derive(
        &self,
        caller: Caller,
        data: ConstructionDeriveRequest,
        mode: &ModeState,
    ) -> Response<ConstructionDeriveResponse> {
        if mode.is_online() {
            ApiError::wrong_network(&data)
        } else {
            self.derive(caller, data).await
        }
    }

    async fn call_hash(
        &self,
        caller: Caller,
        data: ConstructionHashRequest,
        mode: &ModeState,
    ) -> Response<TransactionIdentifierResponse> {
        if mode.is_online() {
            ApiError::wrong_network(&data)
        } else {
            self.hash(caller, data).await
        }
    }

    async fn call_metadata(
        &self,
        caller: Caller,
        data: ConstructionMetadataRequest,
        mode: &ModeState,
    ) -> Response<ConstructionMetadataResponse> {
        if mode.is_offline() {
            ApiError::wrong_network(&data)
        } else {
            self.metadata(caller, data).await
        }
    }

    async fn call_parse(
        &self,
        caller: Caller,
        data: ConstructionParseRequest,
        mode: &ModeState,
    ) -> Response<ConstructionParseResponse> {
        if mode.is_online() {
            ApiError::wrong_network(&data)
        } else {
            self.parse(caller, data).await
        }
    }

    async fn call_payloads(
        &self,
        caller: Caller,
        data: ConstructionPayloadsRequest,
        mode: &ModeState,
    ) -> Response<ConstructionPayloadsResponse> {
        if mode.is_online() {
            ApiError::wrong_network(&data)
        } else {
            self.payloads(caller, data).await
        }
    }

    async fn call_preprocess(
        &self,
        caller: Caller,
        data: ConstructionPreprocessRequest,
        mode: &ModeState,
    ) -> Response<ConstructionPreprocessResponse> {
        if mode.is_online() {
            ApiError::wrong_network(&data)
        } else {
            self.preprocess(caller, data).await
        }
    }

    async fn call_submit(
        &self,
        caller: Caller,
        data: ConstructionSubmitRequest,
        mode: &ModeState,
    ) -> Response<TransactionIdentifierResponse> {
        if mode.is_offline() {
            ApiError::wrong_network(&data)
        } else {
            self.submit(caller, data).await
        }
    }
}
