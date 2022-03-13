use super::*;

/// A BlockRequest is utilized to make a block request on the /block endpoint.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockTransactionRequest {
    /// The network_identifier specifies which network a particular object is
    /// associated with.
    pub network_identifier:     NetworkIdentifier,
    /// The block_identifier uniquely identifies a block in a particular
    /// network.
    pub block_identifier:       BlockIdentifier,
    /// The transaction_identifier uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
}
