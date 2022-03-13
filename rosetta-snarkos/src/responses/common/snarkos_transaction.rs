use mentat::{identifiers::TransactionIdentifier, models::Transaction, IndexMap};

use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosTransaction {
    pub inner_circuit_id: String,
    pub ledger_root: String,
    pub transaction_id: String,
    pub transitions: Vec<SnarkosTransition>,
}

impl From<SnarkosTransaction> for Transaction {
    fn from(transaction: SnarkosTransaction) -> Self {
        Transaction {
            transaction_identifier: TransactionIdentifier {
                hash: transaction.transaction_id,
            },
            operations: transaction
                .transitions
                .into_iter()
                .map(|t| t.into())
                .collect(),
            related_transactions: None,
            // TODO: Size and locktime????
            metadata: IndexMap::new(),
        }
    }
}
