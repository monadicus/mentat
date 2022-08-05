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
        ErrorTest {
            name: "account balance error",
            err: AccountBalanceError::ReturnedBlockHashMismatch.into(),
            is: true,
        },
        ErrorTest {
            name: "block error",
            err: BlockError::BlockIdentifierIsNil.into(),
            is: true,
        },
        ErrorTest {
            name: "coin error",
            err: CoinError::ChangeIsNil.into(),
            is: true,
        },
        ErrorTest {
            name: "construction error",
            err: ConstructionError::ConstructionMetadataResponseIsNil.into(),
            is: true,
        },
        ErrorTest {
            name: "network error",
            err: NetworkError::NetworkIdentifierIsNil.into(),
            is: true,
        },
        ErrorTest {
            name: "server error",
            err: ServerError::NoSupportedNetworks.into(),
            is: true,
        },
        ErrorTest {
            name: "",
            err: Blah {
                content: "blah".to_string(),
            }
            .into(),
            is: false,
        },
    ];

    ErrorTest::run(tests, crate::errors::err);
}
