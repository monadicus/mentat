//! Validates that error data is correct.

use super::{error, AssertResult, ErrorError, MentatError, ResponseAsserter};

impl ResponseAsserter {
    /// `error` ensures a [`MentatError`] matches some error
    /// provided in `/network/options`.
    pub(crate) fn error(&self, err: &MentatError) -> AssertResult<()> {
        // TODO if self nil
        error(err)?;

        let value = self
            .error_type_map
            .get(&err.code)
            .ok_or_else(|| format!("{}: code {}", ErrorError::UnexpectedCode, err.code))?;

        if value.message != err.message {
            Err(format!(
                "{}: expected {} actual {}",
                ErrorError::MessageMismatch,
                value.message,
                err.message,
            ))?;
        }

        if value.retriable != err.retriable {
            Err(format!(
                "{}: expected {} actual {}",
                ErrorError::RetriableMismatch,
                value.message,
                err.message,
            ))?;
        }

        Ok(())
    }
}
