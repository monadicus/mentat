use super::*;

/// A MempoolResponse contains all transaction identifiers in the mempool for a particular network_identifier.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct MempoolResponse {
    pub transaction_identifiers: Vec<TransactionIdentifier>,
}
