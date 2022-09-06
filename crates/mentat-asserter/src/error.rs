//! Validates that error data is correct.

use super::*;

impl Asserter {
    /// `error` ensures a [`UncheckedMentatError`] matches some error
    /// provided in `/network/options`.
    pub fn error(&self, err: Option<&UncheckedMentatError>) -> AssertResult<()> {
        let asserter = self
            .response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        error(err)?;
        let err = err.unwrap();

        let value = asserter
            .error_type_map
            .get(&err.code)
            .ok_or_else(|| format!("code {}: {}", err.code, ErrorError::UnexpectedCode))?;

        if value.message != err.message {
            Err(format!(
                "expected {} actual {}: {}",
                value.message,
                err.message,
                ErrorError::MessageMismatch,
            ))?;
        }

        if value.retriable != err.retriable {
            Err(format!(
                "expected {} actual {}: {}",
                value.message,
                err.message,
                ErrorError::RetriableMismatch,
            ))?;
        }

        Ok(())
    }
}
