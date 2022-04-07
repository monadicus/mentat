use super::*;

/// A BlockResponse includes a fully-populated block or a partially-populated
/// block with a list of other transactions to fetch (other_transactions). As a
/// result of the consensus algorithm of some blockchains, blocks can be omitted
/// (i.e. certain block indices can be skipped). If a query for one of these
/// omitted indices is made, the response should not include a Block object. It
/// is VERY important to note that blocks MUST still form a canonical, connected
/// chain of blocks where each block has a unique index. In other words, the
/// PartialBlockIdentifier of a block after an omitted block should reference
/// the last non-omitted block.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BlockResponse {
    /// Blocks contain an array of Transactions that occurred at a particular
    /// BlockIdentifier. A hard requirement for blocks returned by Rosetta
    /// implementations is that they MUST be inalterable: once a client has
    /// requested and received a block identified by a specific
    /// BlockIndentifier, all future calls for that same BlockIdentifier must
    /// return the same block contents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
    /// Some blockchains may require additional transactions to be fetched that
    /// weren't returned in the block response (ex: block only returns
    /// transaction hashes). For blockchains with a lot of transactions in each
    /// block, this can be very useful as consumers can concurrently fetch all
    /// transactions returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_transactions: Option<Vec<TransactionIdentifier>>,
}
