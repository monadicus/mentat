//! The balance change file contains code for parsing changed balances.

use super::*;
use crate::{Parser, ParserResult};

/// `BalanceChange` represents a balance change that affected
/// a [`AccountIdentifier`] and a [`Currency`].
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BalanceChange {
    /// The account identifier if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    /// The currency if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// The block identifier.
    pub block: BlockIdentifier,
    /// Represents the changed balance of the txs.
    pub difference: String,
}

impl Parser {
    /// `skip_operation` returns a boolean indicating whether
    /// an operation should be processed. An operation will
    /// not be processed if it is considered unsuccessful.
    pub fn skip_operation(&self, op: &Operation) -> ParserResult<bool> {
        // TODO they don't check nil here
        let successful = self.asserter.as_ref().unwrap().operation_successful(op)?;

        if !successful {
            return Ok(true);
        }

        if op.account.is_none() {
            return Ok(true);
        }

        if op.amount.is_none() {
            return Ok(true);
        }

        Ok(self
            .exempt_func
            .as_ref()
            .map(|ef| ef(op))
            .unwrap_or_default())
    }

    /// `balance_changes` returns all balance changes for
    /// a particular block. All balance changes for a
    /// particular account are summed into a single
    /// [`BalanceChange`] struct. If a block is being
    /// orphaned, the opposite of each balance change is
    /// returned.
    pub fn balance_changes(
        &self,
        // TODO how do we replicate this?
        // its for green threading.
        _ctx: (),
        block: &Block,
        block_removed: bool,
    ) -> ParserResult<Vec<BalanceChange>> {
        let mut balance_changes: IndexMap<String, BalanceChange> = IndexMap::new();

        for tx in block.transactions.iter() {
            for op in tx.operations.iter() {
                let skip = self.skip_operation(op)?;

                if skip {
                    // Continue the inner loop.
                    continue;
                }

                // We create a copy of Amount.Value
                // here to ensure we don't accidentally overwrite
                // the value of op.Amount.
                // Safe to unwrap here otherwise we would have skipped.
                let mut amount_value = op.amount.clone().unwrap().value;
                let block_ident = block.block_identifier.clone();

                if block_removed {
                    let negated_value = negate_value(&amount_value)?;
                    amount_value = negated_value;
                }

                let key = format!(
                    "{}/{}",
                    hash(op.account.as_ref()),
                    hash(op.amount.as_ref().map(|amt| amt.currency.clone()).as_ref()),
                );

                let val = balance_changes.get_mut(&key);
                if val.is_none() {
                    balance_changes.insert(
                        key,
                        BalanceChange {
                            account: op.account.clone(),
                            currency: op.amount.as_ref().map(|amt| amt.currency.clone()),
                            block: block_ident,
                            difference: amount_value,
                        },
                    );
                    // Continue the inner loop.
                    continue;
                }

                let mut val = val.unwrap();
                let new_diff = add_values(val.difference.as_ref(), &amount_value)?;
                val.difference = new_diff;
            }
        }

        Ok(balance_changes.into_iter().map(|(_, bc)| bc).collect())
    }
}
