//! Validates that search data is correct.

use super::{
    block_identifier,
    errors::{AsserterError, SearchError},
    AssertResult,
    Asserter,
    SearchTransactionsResponse,
};

impl Asserter {
    /// SearchTransactionsResponse ensures a
    /// *types.SearchTransactionsResponse is valid.
    pub fn search_transaction_response(
        &self,
        response: &SearchTransactionsResponse,
    ) -> AssertResult<()> {
        self.response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        if matches!(response.next_offset, Some(r) if r < 0) {
            Err(SearchError::NextOffsetInvalid)?;
        } else if response.total_count < 0 {
            Err(SearchError::TotalCountInvalid)?;
        }

        response.transactions.iter().try_for_each(|t| {
            // TODO: coinbase never checks for nil here
            let t = t.as_ref().unwrap();
            block_identifier(t.block_identifier.as_ref())?;
            self.transaction(t.transaction.as_ref())
        })
    }
}
