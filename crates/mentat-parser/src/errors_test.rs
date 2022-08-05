use super::*;

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

    ErrorTest::run(tests, crate::errors::err);
}
