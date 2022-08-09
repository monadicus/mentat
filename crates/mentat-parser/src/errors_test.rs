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
        ErrorTest {
            name: "intent error",
            err: IntentError::ExpectedOperationAccountMismatch.into(),
            is: true,
        },
        ErrorTest {
            name: "match operations error",
            err: MatchOperationsError::AccountMatchAccountMissing.into(),
            is: true,
        },
    ];

    ErrorTest::run(tests, err);
}
