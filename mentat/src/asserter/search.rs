//! Validates that search data is correct.

use super::{
    block_identifier,
    errors::SearchError,
    AssertResult,
    ResponseAsserter,
    SearchTransactionsResponse,
};

impl ResponseAsserter {
    /// SearchTransactionsResponse ensures a
    /// *types.SearchTransactionsResponse is valid.
    pub fn search_transaction_response(
        &self,
        response: &SearchTransactionsResponse,
    ) -> AssertResult<()> {
        // TODO if self == nil
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
