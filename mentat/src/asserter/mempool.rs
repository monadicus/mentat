//! Validates that mempool data is correct.

use super::{transaction_identifier, AssertResult, TransactionIdentifier};

/// mempool_transactions returns an error if any
/// [`TransactionIdentifier`] returns is missing a hash.
/// The correctness of each populated [`MempoolTransaction`] is
/// asserted by [`Transaction`].
pub(crate) fn mempool_transactions(transactions: &[TransactionIdentifier]) -> AssertResult<()> {
    transactions.iter().try_for_each(transaction_identifier)
}