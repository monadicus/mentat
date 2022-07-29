//! Validates that error data is correct.

use super::*;

impl Asserter {
    /// `error` ensures a [`MentatError`] matches some error
    /// provided in `/network/options`.
    pub fn error(&self, err: Option<&MentatError>) -> AssertResult<()> {
        let asserter = self
            .response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        error(err)?;
        let err = err.unwrap();

        let value = asserter
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
