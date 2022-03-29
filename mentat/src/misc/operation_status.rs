use super::*;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}
