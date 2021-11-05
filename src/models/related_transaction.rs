use super::*;

/// PublicKey contains a public key byte array for a particular CurveType encoded in hex. Note that there is no PrivateKey struct as this is NEVER the concern of an implementation.
#[derive(Serialize, Deserialize)]
pub struct RelatedTransaction {
    /// The network_identifier specifies which network a particular object is associated with.
    pub network_identifier: Option<NetworkIdentifier>,
    /// The transaction_identifier uniquely identifies a transaction in a particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
    /// Used by RelatedTransaction to indicate the direction of the relation (i.e. cross-shard/cross-network sends may reference backward to an earlier transaction and async execution may reference forward). Can be used to indicate if a transaction relation is from child to parent or the reverse.
    pub direction: Direction,
}