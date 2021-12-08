use super::*;

/// ConstructionCombineRequest is the input to the /construction/combine endpoint. It contains the unsigned transaction blob returned by /construction/payloads and all required signatures to create a network transaction.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionCombineRequest {
    /// The network_identifier specifies which network a particular object is associated with.
    pub network_identifier: NetworkIdentifier,
    pub unsigned_transaction: String,
    pub signatures: Vec<Signature>,
}