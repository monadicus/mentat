use super::*;

#[async_trait::async_trait]
pub trait ConstructionApi: Send + Sync {
    /// Combine creates a network-specific transaction from an unsigned transaction and an array of provided signatures. The signed transaction returned from this method will be sent to the /construction/submit endpoint by the _caller.
    async fn combine(
        &self,
        _caller: Caller,
        data: ConstructionCombineRequest,
        mode: &ModeState,
    ) -> Response<ConstructionCombineResponse> {
        mode.handle(
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
            ApiError::not_implemented,
        )
    }

    /// Derive returns the AccountIdentifier associated with a public key. Blockchains that require an on-chain action to create an account should not implement this method.
    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
        mode: &ModeState,
    ) -> Response<ConstructionDeriveResponse> {
        mode.handle(
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
            ApiError::not_implemented,
        )
    }

    /// TransactionHash returns the network-specific transaction hash for a signed transaction.
    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
        mode: &ModeState,
    ) -> Response<TransactionIdentifierResponse> {
        mode.handle(
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
            ApiError::not_implemented,
        )
    }

    /// Get any information required to construct a transaction for a specific network. Metadata returned here could be a recent hash to use, an account sequence number, or even arbitrary chain state. The request used when calling this endpoint is created by calling /construction/preprocess in an offline environment. You should NEVER assume that the request sent to this endpoint will be created by the _caller or populated with any custom parameters. This must occur in /construction/preprocess. It is important to clarify that this endpoint should not pre-construct any transactions for the client (this should happen in /construction/payloads). This endpoint is left purposely unstructured because of the wide scope of metadata that could be required.
    async fn metadata(
        &self,
        _caller: Caller,
        data: ConstructionMetadataRequest,
        mode: &ModeState,
    ) -> Response<ConstructionMetadataResponse> {
        mode.handle(
            ApiError::not_implemented,
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
        )
    }

    /// Parse is called on both unsigned and signed transactions to understand the intent of the formulated transaction. This is run as a sanity check before signing (after /construction/payloads) and before broadcast (after /construction/combine).
    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
        mode: &ModeState,
    ) -> Response<ConstructionParseResponse> {
        mode.handle(
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
            ApiError::not_implemented,
        )
    }

    /// Payloads is called with an array of operations and the response from /construction/metadata. It returns an unsigned transaction blob and a collection of payloads that must be signed by particular AccountIdentifiers using a certain SignatureType. The array of operations provided in transaction construction often times can not specify all "effects" of a transaction (consider invoked transactions in Ethereum). However, they can deterministically specify the "intent" of the transaction, which is sufficient for construction. For this reason, parsing the corresponding transaction in the Data API (when it lands on chain) will contain a superset of whatever operations were provided during construction.
    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
        mode: &ModeState,
    ) -> Response<ConstructionPayloadsResponse> {
        mode.handle(
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
            ApiError::not_implemented,
        )
    }

    /// Preprocess is called prior to /construction/payloads to construct a request for any metadata that is needed for transaction construction given (i.e. account nonce). The options object returned from this endpoint will be sent to the /construction/metadata endpoint UNMODIFIED by the _caller (in an offline execution environment). If your Construction API implementation has configuration options, they MUST be specified in the /construction/preprocess request (in the metadata field).
    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
        mode: &ModeState,
    ) -> Response<ConstructionPreprocessResponse> {
        mode.handle(
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
            ApiError::not_implemented,
        )
    }

    /// Submit a pre-signed transaction to the node. This call should not block on the transaction being included in a block. Rather, it should return immediately with an indication of whether or not the transaction was included in the mempool. The transaction submission response should only return a 200 status if the submitted transaction could be included in the mempool. Otherwise, it should return an error.
    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        mode: &ModeState,
    ) -> Response<TransactionIdentifierResponse> {
        mode.handle(
            ApiError::not_implemented,
            || ApiError::wrong_network(&data),
            ApiError::not_implemented,
        )
    }
}
