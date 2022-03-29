use super::*;

/// ConstructionCombineResponse is returned by /construction/combine. The
/// network payload will be sent directly to the construction/submit endpoint.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ConstructionCombineResponse {
    pub signed_transaction: String,
}
