use super::*;

/// ConstructionTransactionResponse is returned by /construction/payloads. It contains an unsigned transaction blob (that is usually needed to construct the a network transaction from a collection of signatures) and an array of payloads that must be signed by the caller.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionPayloadsResponse {
    pub unsigned_transaction: String,
    pub payloads: Vec<SigningPayload>,
}
