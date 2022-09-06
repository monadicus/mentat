//! Houses the traits for the Rosetta Block API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// BlockAPIServicer defines the api actions for the BlockAPI service
#[axum::async_trait]
pub trait BlockApi {
    /// the caller used to interact with the underlying node
    type NodeCaller: Send + Sync;

    /// Get a block by its [`crate::identifiers::BlockIdentifier`]. If
    /// transactions are returned in the same call to the node as fetching
    /// the block, the response should include these transactions in the
    /// Block object. If not, an array of [`crate::identifiers::
    /// TransactionIdentifier`]s should be returned so /block/transaction
    /// fetches can be done to get all transaction information. When
    /// requesting a block by the hash component of the
    /// [`crate::identifiers::BlockIdentifier`], this request MUST be
    /// idempotent: repeated invocations for the same hash-identified block
    /// must return the exact same block contents. No such restriction is
    /// imposed when requesting a block by height, given that a chain reorg
    /// event might cause the specific block at height n to be set to a
    /// different one.
    async fn block(
        &self,
        _caller: Caller,
        _data: BlockRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<BlockResponse> {
        MentatError::not_implemented()
    }

    /// Get a transaction in a block by its
    /// [`crate::identifiers::TransactionIdentifier`]. This endpoint should
    /// only be used when querying a node for a block does not return all
    /// transactions contained within it. All transactions returned
    /// by this endpoint must be appended to any transactions returned by the
    /// /block method by consumers of this __data. Fetching a transaction by
    /// hash is considered an Explorer Method (which is classified under the
    /// Future Work section). This method can be used to let consumers to
    /// paginate results when the block transactions count is too big to be
    /// returned in a single [`BlockResponse`]. Calling this endpoint requires
    /// reference to a [`crate::identifiers::BlockIdentifier`] because
    /// transaction parsing can change depending on which block contains the
    /// transaction. For example, in Bitcoin it is necessary to know which
    /// block contains a transaction to determine the destination of fee
    /// payments. Without specifying a block identifier, the node would have
    /// to infer which block to use (which could change during a re-org).
    /// Implementations that require fetching previous transactions to
    /// populate the response (ex: Previous UTXOs in Bitcoin) may find it
    /// useful to run a cache within the Rosetta server in the /__data
    /// directory (on a path that does not conflict with the node).
    async fn block_transaction(
        &self,
        _caller: Caller,
        _data: BlockTransactionRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<BlockTransactionResponse> {
        MentatError::not_implemented()
    }
}

/// BlockAPIRouter defines the required methods for binding the api requests to
/// a responses for the BlockAPI
/// The BlockAPIRouter implementation should parse necessary information from
/// the http request, pass the data to a BlockAPIServicer to perform the
/// required actions, then write the service results to the http response.
#[axum::async_trait]
pub trait BlockApiRouter: BlockApi + Clone + Default {
    /// This endpoint only runs in online mode.
    async fn call_block(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedBlockRequest>,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedBlockResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.block_request(data.as_ref())?;
            let resp: UncheckedBlockResponse = self
                .block(caller, data.unwrap().into(), node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_block_transaction(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedBlockTransactionRequest>,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedBlockTransactionResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.block_transaction_request(data.as_ref())?;
            let resp: UncheckedBlockTransactionResponse = self
                .block_transaction(caller, data.unwrap().into(), node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}
