use super::*;

#[test]
fn test_coin() {
    let valid_amount = NullableAmount {
        value: "1000".to_string(),
        currency: Some(NullableCurrency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    };

    let tests = vec![
        TestCase {
            name: "valid coin",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(valid_amount.clone()),
            }),
            result: None,
        },
        TestCase {
            name: "valid coin",
            payload: None,
            result: Some(CoinError::IsNil.into()),
        },
        TestCase {
            name: "invalid identifier",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                amount: Some(valid_amount),
            }),
            result: Some(AsserterError::from(
                "coin identifier cannot be empty: coin identifier is invalid".to_string(),
            )),
        },
        TestCase {
            name: "invalid amount",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(NullableAmount {
                    value: "100".to_string(),
                    currency: None,
                    metadata: Default::default(),
                }),
            }),
            result: Some(AsserterError::from("amount is invalid".to_string())),
        },
        TestCase {
            name: "nil amount",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: None,
            }),
            result: Some(AsserterError::from("amount is invalid".to_string())),
        },
    ];

    TestCase::run_err_match(tests, |t| coin(t.as_ref()));
}

#[test]
fn test_coins() {
    let valid_amount = NullableAmount {
        value: "1000".to_string(),
        currency: Some(NullableCurrency {
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
                Some(NullableCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                }),
                Some(NullableCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin2".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                }),
            ],
            result: None,
        },
        TestCase {
            name: "nil",
            payload: Vec::new(),
            result: None,
        },
        TestCase {
            name: "duplicate coins",
            payload: vec![
                Some(NullableCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                }),
                Some(NullableCoin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount),
                }),
            ],
            result: Some(CoinError::Duplicate.into()),
        },
    ];

    TestCase::run_err_match(tests, |test| coins(&test));
}

#[test]
fn test_coin_change() {
    let tests = vec![
        TestCase {
            name: "valid change",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: NullableCoinAction::COIN_CREATED.into(),
            }),
            result: None,
        },
        TestCase {
            name: "nil",
            payload: None,
            result: Some(CoinError::ChangeIsNil.into()),
        },
        TestCase {
            name: "invalid identifier",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                coin_action: NullableCoinAction::COIN_CREATED.into(),
            }),
            result: Some(CoinError::IdentifierNotSet.into()),
        },
        TestCase {
            name: "invalid coin action",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: "hello".into(),
            }),
            result: Some(CoinError::ActionInvalid.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| coin_change(t.as_ref()));
}
