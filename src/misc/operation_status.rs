use super::*;

#[derive(Serialize, Deserialize)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}