use super::*;
use crate::api::Caller;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HealthCheckResponse {
    pub caller: Caller,
    pub msg: String,
    pub node_status: Value,
}
