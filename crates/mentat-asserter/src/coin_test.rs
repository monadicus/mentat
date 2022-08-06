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
        FnTest {
            name: "valid coin",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(valid_amount.clone()),
            }),
            result: None,
        },
        FnTest {
            name: "valid coin",
            payload: None,
            result: Some(CoinError::IsNil.into()),
        },
        FnTest {
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
        FnTest {
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
        FnTest {
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

    FnTest::run_err_match(tests, |t| coin(t.as_ref()));
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
        FnTest {
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
        FnTest {
            name: "nil",
            payload: Vec::new(),
            result: None,
        },
        FnTest {
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

    FnTest::run_err_match(tests, |test| coins(&test));
}

#[test]
fn test_coin_change() {
    let tests = vec![
        FnTest {
            name: "valid change",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: NullableCoinAction::COIN_CREATED.into(),
            }),
            result: None,
        },
        FnTest {
            name: "nil",
            payload: None,
            result: Some(CoinError::ChangeIsNil.into()),
        },
        FnTest {
            name: "invalid identifier",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                coin_action: NullableCoinAction::COIN_CREATED.into(),
            }),
            result: Some(CoinError::IdentifierNotSet.into()),
        },
        FnTest {
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

    FnTest::run_err_match(tests, |t| coin_change(t.as_ref()));
}
