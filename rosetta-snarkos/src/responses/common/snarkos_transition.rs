use mentat::{
    identifiers::OperationIdentifier,
    models::{Amount, Currency, Operation},
    IndexMap,
};

use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosTransition {
    pub ciphertexts: Vec<String>,
    pub commitments: Vec<String>,
    pub events: Vec<SnarkosEvent>,
    pub proof: String,
    pub serial_numbers: Vec<String>,
    pub transition_id: String,
    pub value_balance: i32,
}

impl From<SnarkosTransition> for Operation {
    fn from(transition: SnarkosTransition) -> Self {
        Self {
            // TODO: HOW AM I SUPPOSED TA!!!!?????
            operation_identifier: OperationIdentifier {
                index: todo!(),
                network_index: todo!(),
            },
            related_operations: Some(transition.events.into_iter().map(|e| e.into()).collect()),
            type_: todo!(),
            status: None,
            account: None,
            amount: Some(Amount {
                value: transition.value_balance.to_string(),
                currency: Currency {
                    symbol: "ALEO".to_string(),
                    decimals: 18,
                    metadata: IndexMap::new(),
                },
                metadata: IndexMap::new(),
            }),
            coin_change: None,
            metadata: IndexMap::new(),
        }
    }
}
