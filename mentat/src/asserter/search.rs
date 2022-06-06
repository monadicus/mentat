use super::{asserter::ResponseAsserter, block::block_identifier, errors::AssertResult};
use crate::{asserter::errors::SearchError, responses::SearchTransactionsResponse};

impl ResponseAsserter {
    /// SearchTransactionsResponse ensures a
    /// *types.SearchTransactionsResponse is valid.
    pub fn search_transaction_response(
        &self,
        response: &SearchTransactionsResponse,
    ) -> AssertResult<()> {
        // if self == nil
        if matches!(response.next_offset, Some(r) if r < 0) {
            todo!("impossible case");
            return Err(SearchError::NextOffsetInvalid.into());
        } else if response.total_count < 0 {
            todo!("impossible case");
            return Err(SearchError::TotalCountInvalid.into());
        }

        response
            .transactions
            .iter()
            .map(|t| {
                block_identifier(&t.block_identifier)?;
                self.transaction(&t.transaction)
            })
            .collect::<Result<_, _>>()
    }
}
