use super::test_utils::AsserterTest;
use crate::{
    asserter::{
        coin::{coin, coin_change, coins},
        errors::{AsserterError, CoinError},
    },
    types::{Amount, Coin, CoinAction, CoinChange, CoinIdentifier, Currency},
};

#[test]
fn test_coin() {
    let valid_amount = Amount {
        value: "1000".to_string(),
        currency: Some(Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    };

    let tests = [
        AsserterTest {
            name: "valid coin",
            payload: Coin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                amount: Some(valid_amount.clone()),
            },
            err: None,
        },
        // TODO allow None Coin
        // "valid coin" => CoinTest {
        //   coin: None,
        //   err: Some(CoinError::IsNil.into())
        // },
        AsserterTest {
            name: "invalid identifier",
            payload: Coin {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                amount: Some(valid_amount),
            },
            err: Some(AsserterError::from(
                "coin identifier cannot be empty: identifier is invalid".to_string(),
            )),
        },
        // TODO allow None currency
        // "invalid amount" => CoinTest {
        //   coin: Coin {
        //     coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
        //     amount: Amount {
        //       value: "100".to_string(),
        //       currency: None,
        //       metadata: Default::default(),
        //     },
        //   },
        //   err: Some(AsserterError::from("amount is invalid".to_string())),
        // },
        // TODO allow None amount
        // "nil amount" => CoinTest {
        //   coin: Coin {
        //     coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
        //     amount: None,
        //   },
        //   err: Some(AsserterError::from("amount is invalid".to_string()))
        // },
    ];

    AsserterTest::non_asserter_tests(&tests, coin);
}

#[test]
fn test_coins() {
    let valid_amount = Amount {
        value: "1000".to_string(),
        currency: Some(Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    };

    let tests = [
        AsserterTest {
            name: "valid coins",
            payload: vec![
                Coin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                },
                Coin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin2".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                },
            ],
            err: None,
        },
        // TODO allow None coins
        // "nil" => {
        //   coins: None,
        //   err: None,
        // }
        AsserterTest {
            name: "duplicate coins",
            payload: vec![
                Coin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount.clone()),
                },
                Coin {
                    coin_identifier: Some(CoinIdentifier {
                        identifier: "coin1".to_string(),
                    }),
                    amount: Some(valid_amount),
                },
            ],
            err: Some(CoinError::Duplicate.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, |test| coins(test));
}

#[test]
fn test_coin_change() {
    let tests = [
        AsserterTest {
            name: "valid change",
            payload: Some(CoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: "coin1".to_string(),
                }),
                coin_action: CoinAction::CoinCreated,
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
            payload: Some(CoinChange {
                coin_identifier: Some(CoinIdentifier {
                    identifier: String::new(),
                }),
                coin_action: CoinAction::CoinCreated,
            }),
            err: Some(CoinError::IdentifierNotSet.into()),
        },
        // TODO allow arbitrary coin change
        // "invalid coin action" => CoinChangeTest {
        //   change: Some(CoinChange {
        //     coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
        //     coin_action: "hello",
        //   }),
        //   err: Some(CoinError::ActionInvalid.into())
        // },
    ];

    AsserterTest::non_asserter_tests(&tests, |test| coin_change(test.as_ref()));
}
