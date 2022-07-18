use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        coin::{coin, coin_change, coins},
        errors::{AsserterError, CoinError},
    },
    types::{Amount, Coin, CoinAction, CoinChange, CoinIdentifier, Currency},
};

struct CoinTest {
    coin: Coin,
    err: Option<AsserterError>,
}

#[test]
fn test_coin() {
    let valid_amount = Amount {
        value: "1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };

    let tests: IndexMap<&str, CoinTest> = indexmap!(
      "valid coin" => CoinTest {
        coin: Coin {
          coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
          amount: valid_amount.clone(),
        },
        err: None
      },
      // TODO allow None Coin
      // "valid coin" => CoinTest {
      //   coin: None,
      //   err: Some(CoinError::IsNil.into())
      // },
      "invalid identifier" => CoinTest {
        coin: Coin {
          coin_identifier: CoinIdentifier { identifier: String::new() },
          amount: valid_amount,
        },
        err: Some(AsserterError::from("coin identifier cannot be empty: identifier is invalid".to_string()))
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
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = coin(&test.coin);
        if let Err(err) = res {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct CoinsTest {
    coins: Vec<Coin>,
    err: Option<AsserterError>,
}

#[test]
fn test_coins() {
    let valid_amount = Amount {
        value: "1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };

    let tests: IndexMap<&str, CoinsTest> = indexmap!(
      "valid coins" => CoinsTest {
        coins: vec![
          Coin {
            coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
            amount: valid_amount.clone()
          },
          Coin {
            coin_identifier: CoinIdentifier { identifier: "coin2".to_string() },
            amount: valid_amount.clone()
          },
        ],
        err: None
      },
      // TODO allow None coins
      // "nil" => {
      //   coins: None,
      //   err: None,
      // }
      "duplicate coins" => CoinsTest {
        coins: vec![
          Coin {
            coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
            amount: valid_amount.clone()
          },
          Coin {
            coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
            amount: valid_amount
          },
        ],
        err: Some(CoinError::Duplicate.into())
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = coins(&test.coins);
        if let Err(err) = res {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct CoinChangeTest {
    change: Option<CoinChange>,
    err: Option<AsserterError>,
}

#[test]
fn test_coin_change() {
    let tests: IndexMap<&str, CoinChangeTest> = indexmap!(
      "valid change" => CoinChangeTest {
        change: Some(CoinChange {
          coin_identifier: CoinIdentifier { identifier: "coin1".to_string() },
          coin_action: CoinAction::CoinCreated,
        }),
        err: None
      },
      "nil" => CoinChangeTest {
        change: None,
        err: Some(CoinError::ChangeIsNil.into())
      },
      "invalid identifier" => CoinChangeTest {
        change: Some(CoinChange {
          coin_identifier: CoinIdentifier { identifier: String::new() },
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
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = coin_change(test.change.as_ref());
        if let Err(err) = res {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}
