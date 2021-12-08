use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionSubmitRequest {
    /// The network_identifier specifies which network a particular object is associated with.
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}