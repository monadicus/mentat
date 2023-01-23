use std::fmt;

use mentat_test_utils::TestCase;

use crate::errors::KeysError;

// TODO this blah pattern is repeated.
#[derive(Debug)]
struct Blah {
    content: String,
}

impl fmt::Display for Blah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "blah content: {}", self.content)
    }
}

impl std::error::Error for Blah {}

fn err(err: Box<dyn std::error::Error>) -> (bool, &'static str) {
    if err.is::<KeysError>() {
        (true, "keys error")
    } else {
        (false, "")
    }
}

#[test]
fn test_err() {
    let tests = vec![
        TestCase {
            name: "is a keys error",
            payload: KeysError::ErrPrivKeyLengthInvalid.into(),
            criteria: (true, "keys error"),
        },
        TestCase {
            name: "not a keys error",
            payload: Blah {
                content: "blah".to_string(),
            }
            .into(),
            criteria: (false, ""),
        },
    ];

    TestCase::run_output_match(tests, err);
}
