use super::*;

/// A MempoolResponse contains all transaction identifiers in the mempool for a
/// particular network_identifier.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MempoolResponse {
    pub transaction_identifiers: Vec<TransactionIdentifier>,
}
