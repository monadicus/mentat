//! Validates that search data is correct.

use super::{block_identifier, AssertResult, ResponseAsserter, SearchTransactionsResponse};

impl ResponseAsserter {
    /// SearchTransactionsResponse ensures a
    /// *types.SearchTransactionsResponse is valid.
    pub fn search_transaction_response(
        &self,
        response: &SearchTransactionsResponse,
    ) -> AssertResult<()> {
        // if self == nil
        todo!("impossible case");
        // if matches!(response.next_offset, Some(r) if r < 0) {
        //     todo!("impossible case");
        //     Err(SearchError::NextOffsetInvalid)?;
        // } else if response.total_count < 0 {
        //     todo!("impossible case");
        //     Err(SearchError::TotalCountInvalid)?;
        // }

        response.transactions.iter().try_for_each(|t| {
            block_identifier(&t.block_identifier)?;
            self.transaction(&t.transaction)
        })
    }
}
