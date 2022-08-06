//! TODO

use super::*;
/// `OperationGroup` is a group of related operations
/// If all operations in a group have the same operation.Type,
/// the Type is also populated.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct OperationGroup {
    pub type_: String,
    pub operations: Vec<Operation>,
    pub currencies: Vec<Currency>,
    pub nul_amount_present: bool,
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

// TODO actually takes IndexMap<T, Option<T>> and returns Vec<Option<T>>
/// `sort_operation_groups` returns a slice of OperationGroups sorted by the
/// lowest [`OperationIdentifier`].index in each group. This function also sorts
/// all operations in each [`OperationGroup`] by OperationIdentifier.index. It
/// can be useful to consumers to have a deterministic ordering of groups and
/// ops within each group.
pub fn sort_operation_groups(
    op_len: usize,
    op_groups: IndexMap<usize, OperationGroup>,
) -> Vec<OperationGroup> {
    let mut slice_groups = Vec::new();

    // TODO so we need this since our maps are deterministic?
    // Golang map ordering is non-deterministic.
    // Return groups sorted by lowest op in group
    for i in 0..op_len {
        let mut v = if let Some(v) = op_groups.get(&i) {
            v.clone()
        } else {
            continue;
        };

        // Sort all operations by index in a group
        v.operations.sort_by(|a, b| {
            a.operation_identifier
                .index
                .cmp(&b.operation_identifier.index)
        });

        slice_groups.push(v);
    }

    slice_groups
}

// TODO actually returns Vec<Option<T>>
/// `group_operations` parses all of a transaction's operations and returns a
/// slice of each group of related operations (assuming transitive relatedness).
/// This should ONLY be called on operations that have already been asserted for
/// correctness. Assertion ensures there are no duplicate operation indexes,
/// operations are sorted, and that operations only reference operations with
/// an index less than theirs.
///
/// OperationGroups are returned in ascending order based on the lowest
/// [`OperationIdentifier`].index in the group. The operations in each
/// OperationGroup are also sorted.
pub fn group_operations(transaction: &Transaction) -> Vec<OperationGroup> {
    // TODO coinbase passes Nullable Transaction.
    // But it should never be null.
    let ops = &transaction.operations;

    // We use a map of ints to keep track of *OperationGroup instead of a slice
    // because merging groups involves removing and combing many items. While we
    // could manipulate a slice (leaving holes where groups were merged), it
    // seemed less complex to manipulate a map.
    //
    // Nonetheless, either solution avoids modifying up to `n` opAssignments
    // whenever 2 groups merge (this occurs when merging groups in a slice without
    // leaving holes).
    let mut op_groups = IndexMap::new();
    let mut op_assignments = vec![0; ops.len()];
    for (i, op) in ops.iter().enumerate() {
        // Create new group
        if op.related_operations.is_empty() {
            let mut value = OperationGroup {
                type_: op.type_.clone(),
                operations: vec![op.clone()],
                ..Default::default()
            };

            if let Some(amount) = &op.amount {
                value.currencies = vec![amount.currency.clone()];
            } else {
                value.nul_amount_present = true;
            }

            let key = op_groups.len();
            op_groups.insert(key, value).unwrap();
            op_assignments[i] = key as i64;
            continue;
        }

        // Find groups to merge
        let mut groups_to_merge = Vec::new();
        for related_op in &op.related_operations {
            let assignment = op_assignments[related_op.index as usize];
            if !groups_to_merge.contains(&assignment) {
                groups_to_merge.push(assignment)
            }
        }

        // Ensure first index is lowest because all other groups
        // will be merged into it.
        groups_to_merge.sort();

        let merged_group_index = groups_to_merge[0];

        // Add op to unified group
        op_groups[merged_group_index as usize].add_operation_to_group(
            merged_group_index,
            &mut op_assignments,
            op,
        );

        // Merge Groups
        for other_group_index in groups_to_merge[1..].iter() {
            let other_group = &op_groups[*other_group_index as usize];

            // Add otherGroup ops to mergedGroup
            for other_op in other_group.operations.clone() {
                op_groups[merged_group_index as usize].add_operation_to_group(
                    merged_group_index,
                    &mut op_assignments,
                    &other_op,
                )
            }

            // Delete otherGroup
            op_groups.remove(&(*other_group_index as usize)).unwrap();
        }
    }

    sort_operation_groups(ops.len(), op_groups)
}
