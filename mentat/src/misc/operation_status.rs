use super::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}
