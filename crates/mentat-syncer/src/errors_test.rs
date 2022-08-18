#![allow(clippy::missing_docs_in_private_items)]

use std::error::Error;

use mentat_test_utils::TestCase;

use crate::errors::{err, SyncerError};

#[test]
fn test_err() {
    let tests = vec![
        TestCase {
            name: "is a keys error",
            payload: Box::new(SyncerError::CannotRemoveGenesisBlock) as Box<dyn Error>,
            criteria: true,
        },
        TestCase {
            name: "not a keys error",
            payload: "blah".into(),
            criteria: false,
        },
    ];

    TestCase::run_output_match(tests, err)
}
