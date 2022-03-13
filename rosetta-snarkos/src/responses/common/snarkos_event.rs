use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosEvent {
    pub id: u64,
    pub index: u64,
    pub record_view_key: String,
}
