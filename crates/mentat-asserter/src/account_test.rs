use super::*;

#[derive(Default)]
struct ContainsCurrencyTest {
    currencies: Vec<Currency>,
    currency: Currency,
}

#[test]
fn test_contains_currency() {
    let tests = vec![
        TestCase {
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
            criteria: true,
        },
        TestCase {
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
            criteria: true,
        },
        TestCase {
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
            criteria: true,
        },
        TestCase {
            name: "empty",
            payload: ContainsCurrencyTest {
                currencies: Vec::new(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
            },
            criteria: false,
        },
        TestCase {
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
            criteria: false,
        },
        TestCase {
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
            criteria: false,
        },
        TestCase {
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
            criteria: false,
        },
    ];

    TestCase::run_output_match(tests, |test| {
        contains_currency(&test.currencies, &test.currency)
    });
}

#[test]
fn test_contains_duplicate_currency() {
    let tests = vec![
        TestCase {
            name: "simple contains",
            payload: vec![
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
            ],
            criteria: true,
        },
        TestCase {
            name: "complex contains",
            payload: vec![
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }),
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }),
            ],
            criteria: true,
        },
        TestCase {
            name: "more complex contains",
            payload: vec![
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
                }),
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
                }),
            ],
            criteria: true,
        },
        TestCase {
            name: "empty",
            payload: Vec::new(),
            criteria: false,
        },
        TestCase {
            name: "symbol mismatch",
            payload: vec![
                Some(UncheckedCurrency {
                    symbol: "ERX".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
            ],
            criteria: false,
        },
        TestCase {
            name: "decimal mismatch",
            payload: vec![
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 6,
                    metadata: Default::default(),
                }),
            ],
            criteria: false,
        },
        TestCase {
            name: "metadata mismatch",
            payload: vec![
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("hello")),
                }),
                Some(UncheckedCurrency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: indexmap!("blah".to_string() => json!("bye")),
                }),
            ],
            criteria: false,
        },
    ];

    TestCase::run_output_match(tests, |test| {
        contains_duplicate_currency(&test.iter().map(|t| t.as_ref()).collect::<Vec<_>>()).is_some()
    });
}

#[derive(Default)]
struct AccountBalanceTest {
    request_block: Option<UncheckedPartialBlockIdentifier>,
    response_block: UncheckedBlockIdentifier,
    balances: Vec<Option<UncheckedAmount>>,
    _metadata: IndexMap<String, Value>,
}

impl AccountBalanceTest {
    fn run(self) -> AssertResult<()> {
        account_balance_response(
            self.request_block.as_ref(),
            &UncheckedAccountBalanceResponse {
                block_identifier: Some(self.response_block.clone()),
                balances: self.balances.clone(),
                metadata: Default::default(),
            },
        )
    }
}

#[test]
fn test_account_balance() {
    let valid_block = UncheckedBlockIdentifier {
        index: 1000,
        hash: "jsakdl".to_string(),
    };
    let invalid_block = UncheckedBlockIdentifier {
        index: 1,
        hash: String::new(),
    };

    let invalid_index = 1001;
    let invalid_hash = "ajsdk";

    let valid_amt = Some(UncheckedAmount {
        value: "100".to_string(),
        currency: Some(UncheckedCurrency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });

    let tests = vec![
        TestCase {
            name: "simple balance",
            payload: AccountBalanceTest {
                request_block: None,
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            },
            criteria: None,
        },
        TestCase {
            name: "invalid block",
            payload: AccountBalanceTest {
                request_block: None,
                response_block: invalid_block,
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            },
            criteria: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        TestCase {
            name: "duplicate currency",
            payload: AccountBalanceTest {
                request_block: None,
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone(), valid_amt.clone()],
                _metadata: Default::default(),
            },
            criteria: Some(AccountBalanceError::CurrencyUsedMultipleTimes.into()),
        },
        TestCase {
            name: "valid historical request index",
            payload: AccountBalanceTest {
                request_block: Some(UncheckedPartialBlockIdentifier {
                    index: Some(valid_block.index),
                    hash: None,
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            },
            criteria: None,
        },
        TestCase {
            name: "valid historical request hash",
            payload: AccountBalanceTest {
                request_block: Some(UncheckedPartialBlockIdentifier {
                    index: None,
                    hash: Some(valid_block.hash.clone()),
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            },
            criteria: None,
        },
        TestCase {
            name: "invalid historical request index",
            payload: AccountBalanceTest {
                request_block: Some(UncheckedPartialBlockIdentifier {
                    index: Some(invalid_index),
                    hash: Some(valid_block.hash.clone()),
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt.clone()],
                _metadata: Default::default(),
            },
            criteria: Some(AsserterError::from(format!(
                "requested block index {invalid_index} but got {}: {}",
                valid_block.index,
                AccountBalanceError::ReturnedBlockIndexMismatch,
            ))),
        },
        TestCase {
            name: "invalid historical request hash",
            payload: AccountBalanceTest {
                request_block: Some(UncheckedPartialBlockIdentifier {
                    index: Some(valid_block.index),
                    hash: Some(invalid_hash.to_string()),
                }),
                response_block: valid_block.clone(),
                balances: vec![valid_amt],
                _metadata: Default::default(),
            },
            criteria: Some(AsserterError::from(format!(
                "requested block hash {invalid_hash}, but got {}: {}",
                valid_block.hash,
                AccountBalanceError::ReturnedBlockHashMismatch,
            ))),
        },
    ];

    TestCase::run_err_match(tests, AccountBalanceTest::run);
}
