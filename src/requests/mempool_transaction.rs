use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Serialize, Deserialize)]
pub struct MempoolTransactionRequest {
    /// EventsBlocksRequest is utilized to fetch a sequence of BlockEvents indicating which blocks were added and removed from storage to reach the current state.
    pub network_identifier: NetworkIdentifier,
    /// The transaction_identifier uniquely identifies a transaction in a particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
}