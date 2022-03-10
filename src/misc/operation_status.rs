use super::*;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}
