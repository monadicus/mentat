//! The balance change file contains code for parsing changed balances.

use indexmap::IndexMap;
use mentat_types::*;
use serde::{Deserialize, Serialize};

use crate::{Parser, ParserResult};

/// `BalanceChange` represents a balance change that affected
/// a [`AccountIdentifier`] and a [`Currency`].
#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceChange {
    /// The account identifier if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    /// The currency if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<NullableCurrency>,
    /// The block identifier if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockIdentifier>,
    /// Represents the changed balance of the txs.
    pub difference: String,
}

impl<ExemptOperation> Parser<ExemptOperation>
where
    ExemptOperation: Fn(Option<&NullableOperation>) -> bool,
{
    /// `skip_operation` returns a boolean indicating whether
    /// an operation should be processed. An operation will
    /// not be processed if it is considered unsuccessful.
    pub fn skip_operation(&self, op: Option<&NullableOperation>) -> ParserResult<bool> {
        let successful = self.asserter.operation_successful(op)?;

        if !successful {
            return Ok(true);
        }

        // Safe to unwrap
        let op_ref = op.unwrap();

        if op_ref.amount.is_none() {
            return Ok(true);
        }

        if op_ref.amount.is_none() {
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
        block: Option<&NullableBlock>,
        block_removed: bool,
    ) -> ParserResult<Vec<BalanceChange>> {
        let mut balance_changes: IndexMap<String, BalanceChange> = IndexMap::new();

        // TODO they don't check for nil here
        for tx in block
            .map(|b| b.transactions.as_slice())
            .into_iter()
            .flatten()
        {
            for op in tx
                .as_ref()
                .map(|tx| tx.operations.as_slice())
                .into_iter()
                .flatten()
            {
                let skip = self.skip_operation(op.as_ref())?;

                if skip {
                    // Continue the inner loop.
                    continue;
                }

                // We create a copy of Amount.Value
                // here to ensure we don't accidentally overwrite
                // the value of op.Amount.
                let mut amount_value = op
                    .as_ref()
                    .and_then(|op| op.amount.as_ref().map(|amt| amt.value.clone()))
                    .unwrap_or_else(|| "0".to_string());
                let block_ident = block.unwrap().block_identifier.clone();

                if block_removed {
                    let negated_value = negate_value(&amount_value)?;
                    amount_value = negated_value;
                }

                let key = format!(
                    "{}/{}",
                    hash(op.as_ref().and_then(|op| op.amount.as_ref())),
                    hash(op.as_ref().and_then(|op| {
                        op.amount.as_ref().and_then(|amt| amt.currency.as_ref())
                    })),
                );

                let val = balance_changes.get_mut(&key);
                if val.is_none() {
                    balance_changes.insert(
                        key,
                        BalanceChange {
                            account: op.as_ref().and_then(|op| op.account.clone()),
                            currency: op.as_ref().and_then(|op| {
                                op.amount.as_ref().and_then(|amt| amt.currency.clone())
                            }),
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
