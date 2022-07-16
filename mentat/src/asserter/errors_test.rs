use std::{error::Error, fmt::Display};

use indexmap::{indexmap, IndexMap};

use crate::asserter::errors::*;

struct ErrTest {
    err: Box<dyn Error>,
    is: bool,
    source: String,
}

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
    let tests: IndexMap<&str, ErrTest> = indexmap!(
      "account balance error" => ErrTest {
        err: AccountBalanceError::ReturnedBlockHashMismatch.into(),
        is: true,
        source: "account balance error".to_string(),
      },
      "block error" => ErrTest {
        err: BlockError::BlockIdentifierIsNil.into(),
        is: true,
        source: "block error".to_string(),
      },
      "coin error" => ErrTest {
        err: CoinError::ChangeIsNil.into(),
        is: true,
        source: "coin error".to_string(),
      },
      "construction error" => ErrTest {
        err: ConstructionError::ConstructionMetadataResponseIsNil.into(),
        is: true,
        source: "construction error".to_string(),
      },
      "network error" => ErrTest {
        err: NetworkError::NetworkIdentifierIsNil.into(),
        is: true,
        source: "network error".to_string(),
      },
      "server error" => ErrTest {
        err: ServerError::NoSupportedNetworks.into(),
        is: true,
        source: "server error".to_string(),
      },
      "not an assert error" => ErrTest {
        err: Blah {
          content: "blah".to_string()
        }.into(),
        is: false,
        source: String::new(),
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let (is, source) = err(test.err);
        assert_eq!(is, test.is);
        assert_eq!(source, &test.source);
    });
}
