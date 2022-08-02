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

    let tests = [
        AsserterTest {
            name: "valid coin",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(valid_amount.clone()),
            }),
            err: None,
        },
        AsserterTest {
            name: "valid coin",
            payload: None,
            err: Some(CoinError::IsNil.into()),
        },
        AsserterTest {
            name: "invalid identifier",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                amount: Some(valid_amount),
            }),
            err: Some(AsserterError::from(
                "coin identifier cannot be empty: coin identifier is invalid".to_string(),
            )),
        },
        AsserterTest {
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
            err: Some(AsserterError::from("amount is invalid".to_string())),
        },
        AsserterTest {
            name: "nil amount",
            payload: Some(NullableCoin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: None,
            }),
            err: Some(AsserterError::from("amount is invalid".to_string())),
        },
    ];

    AsserterTest::run(&tests, coin);
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

    let tests = [
        AsserterTest {
            name: "valid coins",
            payload: Some(vec![
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
            ]),
            err: None,
        },
        AsserterTest {
            name: "nil",
            payload: Some(Vec::new()),
            err: None,
        },
        AsserterTest {
            name: "duplicate coins",
            payload: Some(vec![
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
            ]),
            err: Some(CoinError::Duplicate.into()),
        },
    ];

    // TODO: remove use of Some
    AsserterTest::run(&tests, |test| coins(test.unwrap()));
}

#[test]
fn test_coin_change() {
    let tests = [
        AsserterTest {
            name: "valid change",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: NullableCoinAction::COIN_CREATED.into(),
            }),
            err: None,
        },
        AsserterTest {
            name: "nil",
            payload: None,
            err: Some(CoinError::ChangeIsNil.into()),
        },
        AsserterTest {
            name: "invalid identifier",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                coin_action: NullableCoinAction::COIN_CREATED.into(),
            }),
            err: Some(CoinError::IdentifierNotSet.into()),
        },
        AsserterTest {
            name: "invalid coin action",
            payload: Some(NullableCoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: "hello".into(),
            }),
            err: Some(CoinError::ActionInvalid.into()),
        },
    ];

    AsserterTest::run(&tests, coin_change);
}
