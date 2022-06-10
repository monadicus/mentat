use indexmap::{indexmap, IndexMap};
use num_bigint_dig::ParseBigIntError;
use serde_json::{json, Value};

use crate::{
    asserter::{
        account::{account_balance_response, contains_currency, contains_duplicate_currency},
        errors::{AccountBalanceError, AsserterError, BlockError},
    },
    errors::MentatError,
    identifiers::{BlockIdentifier, PartialBlockIdentifier},
    models::{Amount, Currency},
    responses::AccountBalanceResponse,
};

struct ContainsCurrencyTest {
    currencies: Vec<Currency>,
    currency: Currency,
    contains: bool,
}

#[test]
fn test_contains_currency() {
    let tests: IndexMap<&str, ContainsCurrencyTest> = indexmap!(
        "simple contains" => ContainsCurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            },
            contains: true,
        },
        "complex contains" => ContainsCurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("hello")),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("hello")),
            },
            contains: true,
        },
        "more complex contains" => ContainsCurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
            },
            contains: true,
        },
        "empty" => ContainsCurrencyTest {
            currencies: Vec::new(),
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            },
            contains: false,
        },
        "symbol mismatch" => ContainsCurrencyTest {
            currencies: vec![Currency {
                symbol: "ERX".to_string(),
                decimals: 8,
                metadata: Default::default(),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            },
            contains: false,
        },
        "decimal mismatch" => ContainsCurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 6,
                metadata: Default::default(),
            },
            contains: false,
        },
        "metadata mismatch" => ContainsCurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("hello")),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("bye")),
            },
            contains: false,
        },
    );

    tests.iter().for_each(|(name, test)| {
        println!("{name}");
        let exists = contains_currency(&test.currencies, &test.currency);
        assert_eq!(test.contains, exists);
    });
}

struct ContainsDuplicateCurrencyTest {
    currencies: Vec<Currency>,
    duplicate: bool,
}

#[test]
fn test_contains_duplicate_currency() {
    let tests: IndexMap<&str, ContainsDuplicateCurrencyTest> = indexmap!(
        "simple contains" => ContainsDuplicateCurrencyTest {
            currencies: vec![
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
            ],
            duplicate: true,
        },
        "complex contains" => ContainsDuplicateCurrencyTest {
            currencies: vec![
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                },
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }
            ],
            duplicate: true,
        },
        "more complex contains" => ContainsDuplicateCurrencyTest {
            currencies: vec![
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
                },
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
                },
            ],
            duplicate: true,
        },
        "empty" => ContainsDuplicateCurrencyTest {
            currencies: Vec::new(),
            duplicate: false,
        },
        "symbol mismatch" => ContainsDuplicateCurrencyTest {
            currencies: vec![
                Currency {
                    symbol: "ERX".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
            ],

            duplicate: false,
        },
        "decimal mismatch" => ContainsDuplicateCurrencyTest {
            currencies: vec![
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 6,
                    metadata: Default::default(),
                },
            ],
            duplicate: false,
        },
        "metadata mismatch" => ContainsDuplicateCurrencyTest {
            currencies: vec![
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                },
                Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("bye")),
                },
            ],
            duplicate: false,
        },
    );

    tests.iter().for_each(|(name, test)| {
        println!("{name}");
        let exists = contains_duplicate_currency(&test.currencies);
        assert_eq!(test.duplicate, exists.is_some());
    });
}

struct AccountBalanceTest {
    request_block: Option<PartialBlockIdentifier>,
    response_block: BlockIdentifier,
    balances: Vec<Amount>,
    metadata: IndexMap<String, Value>,
    err: Option<AsserterError>,
}

#[test]
fn test_account_balance() {
    let valid_block = BlockIdentifier {
        index: 1000,
        hash: "jsakdl".to_string(),
    };
    let invalid_block = BlockIdentifier {
        index: 1,
        hash: String::new(),
    };

    let invalid_index = 1001;
    let invalid_hash = "ajsdk";

    let valid_amt = Amount {
        value: "100".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };

    let tests: IndexMap<&str, AccountBalanceTest> = indexmap!(
        "simple balance" => AccountBalanceTest {
            request_block: None,
            response_block: valid_block.clone(),
            balances: vec![valid_amt.clone()],
            metadata: Default::default(),
            err: None
        },
        "invalid block" => AccountBalanceTest {
            request_block: None,
            response_block: invalid_block,
            balances: vec![valid_amt.clone()],
            metadata: Default::default(),
            err: Some(AsserterError::from(format!("{}: block identifier is invalid", BlockError::BlockIdentifierHashMissing))),
        },
        "duplicate currency" => AccountBalanceTest {
            request_block: None,
            response_block: valid_block.clone(),
            balances: vec![
                valid_amt.clone(),
                valid_amt.clone(),
            ],
            metadata: Default::default(),
            err: Some(AsserterError::from(format!("currency {:?} used multiple times: balance amounts are invalid", &valid_amt.currency))),
        },
        "valid historical request index" => AccountBalanceTest {
            request_block: Some(PartialBlockIdentifier {
                index: Some(valid_block.index),
                hash: None,
            }),
            response_block: valid_block.clone(),
            balances: vec![valid_amt.clone()],
            metadata: Default::default(),
            err: None
        },
        "valid historical request hash" => AccountBalanceTest {
            request_block: Some(PartialBlockIdentifier {
                index: None,
                hash: Some(valid_block.hash.clone()),
            }),
            response_block: valid_block.clone(),
            balances: vec![valid_amt.clone()],
            metadata: Default::default(),
            err: None
        },
        "invalid historical request index" => AccountBalanceTest {
            request_block: Some(PartialBlockIdentifier {
                index: Some(invalid_index),
                hash: Some(valid_block.hash.clone()),
            }),
            response_block: valid_block.clone(),
            balances: vec![valid_amt.clone()],
            metadata: Default::default(),
            err: Some(AsserterError::from(format!(
                "{}: requested block index {invalid_index} but got {}",
                AccountBalanceError::ReturnedBlockIndexMismatch,
                valid_block.index,
            )))
        },
        "invalid historical request hash" => AccountBalanceTest {
            request_block: Some(PartialBlockIdentifier {
                index: Some(valid_block.index),
                hash: Some(invalid_hash.to_string()),
            }),
            response_block: valid_block.clone(),
            balances: vec![valid_amt],
            metadata: Default::default(),
            err: Some(AsserterError::from(format!(
                "{}: requested block hash {invalid_hash} but got {}",
                AccountBalanceError::ReturnedBlockHashMismatch,
                valid_block.hash,
            )))
        },
    );

    tests.iter().for_each(|(name, test)| {
        println!("{name}");
        let resp = account_balance_response(
            test.request_block.as_ref(),
            &AccountBalanceResponse {
                block_identifier: test.response_block.clone(),
                balances: test.balances.clone(),
                metadata: Default::default(),
            },
        );

        if let Err(err) = resp {
            assert_eq!(
                Some(err.to_string()),
                test.err.as_ref().map(|e| e.to_string())
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}
