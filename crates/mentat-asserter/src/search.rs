//! Validates that search data is correct.

use super::*;

impl Asserter {
    /// SearchTransactionsResponse ensures a
    /// *types.SearchTransactionsResponse is valid.
    pub fn search_transaction_response(
        &self,
        response: Option<&UncheckedSearchTransactionsResponse>,
    ) -> AssertResult<()> {
        self.response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        // TODO coinbase doesn't check for nil here.
        let response = response.unwrap();

        if matches!(response.next_offset, Some(r) if r < 0) {
            Err(SearchError::NextOffsetInvalid)?;
        } else if response.total_count < 0 {
            Err(SearchError::TotalCountInvalid)?;
        }

        response.transactions.iter().try_for_each(|t| {
            // TODO: coinbase never checks for nil here
            let t = t.as_ref().unwrap();
            block_identifier(t.block_identifier.as_ref()).map_err(|e| {
                format!("block identifier {:?} is invalid: {e}", t.block_identifier)
            })?;
            self.transaction(t.transaction.as_ref())
                .map_err(|e| format!("transaction {:?} is invalid: {e}", t.transaction).into())
        })
    }
}
