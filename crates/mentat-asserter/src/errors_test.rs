//! these tests are useless in Rust, but added anyways so we can say we match
//! the original Go tests

use std::{error::Error, fmt::Display};

use super::*;

#[derive(Debug)]
struct Blah {
    content: String,
}

impl Display for Blah {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "blah content: {}", self.content)
    }
}

impl Error for Blah {}

#[test]
fn test_err() {
    let tests = vec![
        TestCase {
            name: "account balance error",
            payload: AccountBalanceError::ReturnedBlockHashMismatch.into(),
            criteria: (true, "account balance error"),
        },
        TestCase {
            name: "block error",
            payload: BlockError::BlockIdentifierIsNil.into(),
            criteria: (true, "block error"),
        },
        TestCase {
            name: "coin error",
            payload: CoinError::ChangeIsNil.into(),
            criteria: (true, "coin error"),
        },
        TestCase {
            name: "construction error",
            payload: ConstructionError::ConstructionMetadataResponseIsNil.into(),
            criteria: (true, "construction error"),
        },
        TestCase {
            name: "network error",
            payload: NetworkError::NetworkIdentifierIsNil.into(),
            criteria: (true, "network error"),
        },
        TestCase {
            name: "server error",
            payload: ServerError::NoSupportedNetworks.into(),
            criteria: (true, "server error"),
        },
        TestCase {
            name: "",
            payload: Blah {
                content: "blah".to_string(),
            }
            .into(),
            criteria: (false, ""),
        },
    ];

    TestCase::run_output_match(tests, crate::errors::err);
}
