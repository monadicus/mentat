use mentat::{
    identifiers::{CoinIdentifier, OperationIdentifier},
    indexmap::IndexMap,
    models::{Amount, CoinAction, CoinChange, Currency, Operation},
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
            operation_identifier: OperationIdentifier {
                index: 0,
                network_index: None,
            },
            related_operations: Some(transition.events.into_iter().map(|e| e.into()).collect()),
            // TODO: I see no information on this.
            type_: "N/A".to_string(),
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
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: transition.transition_id,
                },
                // TODO: I see no information on this.
                coin_action: CoinAction::CoinCreated,
            }),
            metadata: IndexMap::new(),
        }
    }
}
