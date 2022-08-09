//! these tests are useless in Rust, but added anyways so we can say we match
//! the original Go tests

use super::*;

/// `err` takes an error as an argument and returns
/// whether or not the error is one thrown by the asserter
/// along with the specific source of the error
pub fn err(err: Box<dyn std::error::Error>) -> (bool, &'static str) {
    if err.is::<IntentError>() {
        (true, "account balance error")
    } else if err.is::<MatchOperationsError>() {
        (true, "match error")
    } else {
        (false, "")
    }
}

#[test]
fn test_err() {
    let tests = vec![
        TestCase {
            name: "intent error",
            payload: IntentError::ExpectedOperationAccountMismatch.into(),
            criteria: (true, "intent error"),
        },
        TestCase {
            name: "match operations error",
            payload: MatchOperationsError::AccountMatchAccountMissing.into(),
            criteria: (true, "match operations error"),
        },
    ];

    TestCase::run_output_match(tests, err);
}
