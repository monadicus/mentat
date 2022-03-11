use super::*;

/// ConstructionHashRequest is the input to the /construction/hash endpoint.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct ConstructionHashRequest {
    /// The network_identifier specifies which network a particular object is associated with.
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}
