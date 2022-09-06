//! Houses the traits for the Rosetta Mempool API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// MempoolAPIServicer defines the api actions for the MempoolAPI service
#[axum::async_trait]
pub trait MempoolApi {
    /// the caller used to interact with the underlying node
    type NodeCaller: Send + Sync;

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

/// MempoolAPIRouter defines the required methods for binding the api requests
/// to a responses for the MempoolAPI
/// The MempoolAPIRouter implementation should parse necessary information from
/// the http request, pass the data to a MempoolAPIServicer to perform the
/// required actions, then write the service results to the http response.
#[axum::async_trait]
pub trait MempoolApiRouter: MempoolApi + Clone + Default {
    /// This endpoint only runs in online mode.
    async fn call_mempool(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedNetworkRequest>,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedMempoolResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.network_request(data.as_ref())?;
            let resp: UncheckedMempoolResponse = self
                .mempool(caller, data.unwrap().into(), node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_mempool_transaction(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedMempoolTransactionRequest>,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedMempoolTransactionResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.mempool_transaction_request(data.as_ref())?;
            let resp: UncheckedMempoolTransactionResponse = self
                .mempool_transaction(caller, data.unwrap().into(), node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}
