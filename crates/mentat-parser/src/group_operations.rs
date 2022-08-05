//! TODO

use super::*;
/// `OperationGroup` is a group of related operations
/// If all operations in a group have the same operation.Type,
/// the Type is also populated.
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct OperationGroup {
    type_: String,
    operations: Vec<Operation>,
    currencies: Vec<Currency>,
    nul_amount_present: bool,
}

impl OperationGroup {
    /// `add_operation_to_group` appends a [`Operation`] to an
    /// [`OperationGroup`].
    fn add_operation_to_group(
        &mut self,
        destination_index: i64,
        assignments: &mut [i64],
        op: &Operation,
    ) {
        // Remove group type if different
        if !self.type_.is_empty() && op.type_ != self.type_ {
            self.type_ = String::new();
        }

        // Update op assignment
        self.operations.push(op.clone());
        // Safe to do since asserter has checked.
        assignments[op.operation_identifier.index as usize] = destination_index;

        // Handle nil currency
        if op.amount.is_none() {
            self.nul_amount_present = true;
            return;
        }

        // Add op to currency if amount is not nil
        if !contains_currency(&self.currencies, &op.amount.as_ref().unwrap().currency) {
            self.currencies.push(op.amount.clone().unwrap().currency);
        }
    }
}
