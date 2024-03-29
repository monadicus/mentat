//! Validates that mempool data is correct.

use super::*;

/// mempool_transactions returns an error if any
/// [`TransactionIdentifier`] returns is missing a hash.
/// The correctness of each populated [`MempoolTransaction`] is
/// asserted by [`Transaction`].
pub fn mempool_transactions(transactions: &[Option<TransactionIdentifier>]) -> AssertResult<()> {
    transactions.iter().try_for_each(|t| {
        transaction_identifier(t.as_ref())
            .map_err(|e| format!("transaction identifier {t:?} is invalid: {e}").into())
    })
}
