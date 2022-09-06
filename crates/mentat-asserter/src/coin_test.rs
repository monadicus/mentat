use super::*;

#[test]
fn test_coin() {
    let valid_amount = UncheckedAmount {
        value: "1000".to_string(),
        currency: Some(UncheckedCurrency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    };

    let tests = vec![
        TestCase {
            name: "valid coin",
            payload: Some(UncheckedCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(valid_amount.clone()),
            }),
            criteria: None,
        },
        TestCase {
            name: "nil",
            payload: None,
            criteria: Some(CoinError::IsNil.into()),
        },
        TestCase {
            name: "invalid identifier",
            payload: Some(UncheckedCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                amount: Some(valid_amount),
            }),
            criteria: Some(CoinError::IdentifierNotSet.into()),
        },
        TestCase {
            name: "invalid amount",
            payload: Some(UncheckedCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(UncheckedAmount {
                    value: "100".to_string(),
                    currency: None,
                    metadata: Default::default(),
                }),
            }),
            criteria: Some(BlockError::AmountCurrencyIsNil.into()),
        },
        TestCase {
            name: "nil amount",
            payload: Some(UncheckedCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: None,
            }),
            criteria: Some(BlockError::AmountValueMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| coin(t.as_ref()));
}

#[test]
fn test_coins() {
    let valid_amount = UncheckedAmount {
        value: "1000".to_string(),
        currency: Some(UncheckedCurrency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    };

    let tests = vec![
        TestCase {
            name: "valid coins",
            payload: vec![
                Some(UncheckedCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                }),
                Some(UncheckedCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin2".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                }),
            ],
            criteria: None,
        },
        TestCase {
            name: "nil",
            payload: Vec::new(),
            criteria: None,
        },
        TestCase {
            name: "duplicate coins",
            payload: vec![
                Some(UncheckedCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                }),
                Some(UncheckedCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount),
                }),
            ],
            criteria: Some(CoinError::Duplicate.into()),
        },
    ];

    TestCase::run_err_match(tests, |test| coins(&test));
}

#[test]
fn test_coin_change() {
    let tests = vec![
        TestCase {
            name: "valid change",
            payload: Some(UncheckedCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: UncheckedCoinAction::COIN_CREATED.into(),
            }),
            criteria: None,
        },
        TestCase {
            name: "nil",
            payload: None,
            criteria: Some(CoinError::ChangeIsNil.into()),
        },
        TestCase {
            name: "invalid identifier",
            payload: Some(UncheckedCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                coin_action: UncheckedCoinAction::COIN_CREATED.into(),
            }),
            criteria: Some(CoinError::IdentifierNotSet.into()),
        },
        TestCase {
            name: "invalid coin action",
            payload: Some(UncheckedCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: "hello".into(),
            }),
            criteria: Some(CoinError::ActionInvalid.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| coin_change(t.as_ref()));
}
