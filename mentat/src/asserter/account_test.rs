use indexmap::{indexmap, IndexMap};
use serde_json::{json, Value};

use super::*;

#[derive(Default)]
struct ContainsCurrencyTest {
    currencies: Vec<Currency>,
    currency: Currency,
}

#[test]
fn test_contains_currency() {
    let tests = [
        AsserterEqualityTest {
            name: "simple contains",
            payload: ContainsCurrencyTest {
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
            },
            res: true,
        },
        AsserterEqualityTest {
            name: "complex contains",
            payload: ContainsCurrencyTest {
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
            },
            res: true,
        },
        AsserterEqualityTest {
            name: "more complex contains",
            payload: ContainsCurrencyTest {
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
            },
            res: true,
        },
        AsserterEqualityTest {
            name: "empty",
            payload: ContainsCurrencyTest {
                currencies: Vec::new(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
            },
            res: false,
        },
        AsserterEqualityTest {
            name: "symbol mismatch",
            payload: ContainsCurrencyTest {
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
            },
            res: false,
        },
        AsserterEqualityTest {
            name: "decimal mismatch",
            payload: ContainsCurrencyTest {
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
            },
            res: false,
        },
        AsserterEqualityTest {
            name: "metadata mismatch",
            payload: ContainsCurrencyTest {
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
            },
            res: false,
        },
    ];

    AsserterEqualityTest::run(&tests, |test| {
        contains_currency(&test.currencies, &test.currency)
    });
}

#[test]
fn test_contains_duplicate_currency() {
    let tests = [
        AsserterEqualityTest {
            name: "simple contains",
            payload: vec![
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
            ],
            res: true,
        },
        AsserterEqualityTest {
            name: "complex contains",
            payload: vec![
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }),
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }),
            ],
            res: true,
        },
        AsserterEqualityTest {
            name: "more complex contains",
            payload: vec![
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
                }),
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
                }),
            ],
            res: true,
        },
        AsserterEqualityTest {
            name: "empty",
            payload: Vec::new(),
            res: false,
        },
        AsserterEqualityTest {
            name: "symbol mismatch",
            payload: vec![
                Some(Currency {
                    symbol: "ERX".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
            ],
            res: false,
        },
        AsserterEqualityTest {
            name: "decimal mismatch",
            payload: vec![
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 6,
                    metadata: Default::default(),
                }),
            ],
            res: false,
        },
        AsserterEqualityTest {
            name: "metadata mismatch",
            payload: vec![
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }),
                Some(Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("bye")),
                }),
            ],
            res: false,
        },
    ];

    AsserterEqualityTest::run(&tests, |test| {
        contains_duplicate_currency(&test.iter().map(|t| t.as_ref()).collect::<Vec<_>>()).is_some()
    });
}

#[derive(Default)]
struct AccountBalanceTest {
    request_block: Option<PartialBlockIdentifier>,
    response_block: BlockIdentifier,
    balances: Vec<Option<Amount>>,
    _metadata: IndexMap<String, Value>,
}

impl AccountBalanceTest {
    fn run(&self) -> AssertResult<()> {
        account_balance_response(
            self.request_block.as_ref(),
            &AccountBalanceResponse {
                block_identifier: Some(self.response_block.clone()),
                balances: self.balances.clone(),
                metadata: Default::default(),
            },
        )
    }
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

    let valid_amt = Some(Amount {
        value: "100".to_string(),
        currency: Some(Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });

    let tests = [
        AsserterTest {
            name: "simple balance",
            payload: Some(AccountBalanceTest {
                request_block: None,
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid block",
            payload: Some(AccountBalanceTest {
                request_block: None,
                response_block: invalid_block,
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "{}: block identifier is invalid",
                BlockError::BlockIdentifierHashMissing
            ))),
        },
        AsserterTest {
            name: "duplicate currency",
            payload: Some(AccountBalanceTest {
                request_block: None,
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone(), valid_amt.clone()],
                _metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "currency {:?} used multiple times: balance amounts are invalid",
                &valid_amt.as_ref().unwrap().currency
            ))),
        },
        AsserterTest {
            name: "valid historical request index",
            payload: Some(AccountBalanceTest {
                request_block: Some(PartialBlockIdentifier {
                    index: Some(valid_block.index),
                    hash: None,
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "valid historical request hash",
            payload: Some(AccountBalanceTest {
                request_block: Some(PartialBlockIdentifier {
                    index: None,
                    hash: Some(valid_block.hash.clone()),
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid historical request index",
            payload: Some(AccountBalanceTest {
                request_block: Some(PartialBlockIdentifier {
                    index: Some(invalid_index),
                    hash: Some(valid_block.hash.clone()),
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "{}: requested block index {invalid_index} but got {}",
                AccountBalanceError::ReturnedBlockIndexMismatch,
                valid_block.index,
            ))),
        },
        AsserterTest {
            name: "invalid historical request hash",
            payload: Some(AccountBalanceTest {
                request_block: Some(PartialBlockIdentifier {
                    index: Some(valid_block.index),
                    hash: Some(invalid_hash.to_string()),
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt],
                _metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "{}: requested block hash {invalid_hash} but got {}",
                AccountBalanceError::ReturnedBlockHashMismatch,
                valid_block.hash,
            ))),
        },
    ];

    AsserterTest::run(&tests, |t| t.unwrap().run());
}
