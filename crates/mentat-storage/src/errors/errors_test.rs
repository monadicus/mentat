use std::{error::Error, fmt};

use mentat_test_utils::TestCase;

use super::{BalanceStorageError, BlockStorageError, KeyStorageError, StorageErrorType};

#[derive(Debug)]
enum TestError {
    Blah,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for TestError {}

#[test]
fn test_err() {
    let tests = vec![
        TestCase {
            name: "balance storage error",
            payload: Box::new(BalanceStorageError::NegativeBalance) as Box<dyn Error>,
            criteria: Some(StorageErrorType::Balance),
        },
        TestCase {
            name: "block storage error",
            payload: Box::new(BlockStorageError::HeadBlockNotFound) as Box<dyn Error>,
            criteria: Some(StorageErrorType::Block),
        },
        TestCase {
            name: "key storage error",
            payload: Box::new(KeyStorageError::AddrExists) as Box<dyn Error>,
            criteria: Some(StorageErrorType::Key),
        },
        TestCase {
            name: "not a storage error",
            payload: Box::new(TestError::Blah) as Box<dyn Error>,
            criteria: None,
        },
    ];

    TestCase::run_output_match(tests, StorageErrorType::check)
}
