use mentat::identifiers::OperationIdentifier;

use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosEvent {
    pub id: u64,
    pub index: u64,
    pub record_view_key: String,
}

impl From<SnarkosEvent> for OperationIdentifier {
    fn from(event: SnarkosEvent) -> Self {
        Self {
            index: event.index,
            network_index: Some(event.id),
        }
    }
}
