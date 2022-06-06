use super::{block::transaction_identifier, errors::AssertResult};
use crate::identifiers::TransactionIdentifier;

/// mempool_transactions returns an error if any
/// [`TransactionIdentifier`] returns is missing a hash.
/// The correctness of each populated [`MempoolTransaction`] is
/// asserted by [`Transaction`].
pub(crate) fn mempool_transactions(transactions: &[TransactionIdentifier]) -> AssertResult<()> {
    transactions
        .iter()
        .map(|t| transaction_identifier(t))
        .collect::<Result<_, _>>()
}
