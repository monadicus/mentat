use std::fmt;

use indexmap::indexmap;
use mentat_test_utils::*;
use serde_json::json;

use super::*;

#[test]
fn test_construct_partial_block_identifier() {
    let block_identifier = BlockIdentifier {
        index: 1,
        hash: "block 1".into(),
    };

    let partial_block_identifier = PartialBlockIdentifier {
        index: Some(block_identifier.index),
        hash: Some(block_identifier.hash.clone()),
    };

    assert_eq!(
        partial_block_identifier,
        construct_partialblock_identifier(&block_identifier)
    )
}

#[test]
fn test_hash() {
    let ai = Some(AccountIdentifier {
        address: "foo".into(),
        sub_account: None,
        metadata: indexmap!(
            "a".to_string() => "b".into(),
            "b".to_string() => "c".into(),
            "c".to_string() => "d".into(),
            "blahz".to_string() => json!({
                "test": 6,
                "wha": json!({
                    "sweet": 3,
                    "nice": true,
                }),
            }),
            "d".to_string() => json!({
                "t": "p",
                "e": 2,
                "k": "1",
                "blah": json!({
                    "test": 2,
                    "neat": "hello",
                    "cool": json!({
                        "sweet": 3,
                        "nice": true,
                    })
                })
            })
        ),
    });
    let hashed = hash(ai.as_ref());
    assert_eq!(hashed, hash(ai.as_ref()));
}

#[test]
fn test_add_values() {
    let tests = vec![
        TestCase {
            name: "simple",
            payload: ("1", "1"),
            criteria: Ok("2".into()),
        },
        TestCase {
            name: "large",
            payload: (
                "1000000000000000000000000",
                "100000000000000000000000000000000",
            ),
            criteria: Ok("100000001000000000000000000000000".into()),
        },
        TestCase {
            name: "decimal",
            payload: (
                "10000000000000000000000.01",
                "100000000000000000000000000000000",
            ),
            criteria: Err("10000000000000000000000.01 is not an integer".into()),
        },
        TestCase {
            name: "negative",
            payload: ("-13213", "12332"),
            criteria: Ok("-881".into()),
        },
        TestCase {
            name: "invalid number",
            payload: ("-13213", "hello"),
            criteria: Err("hello is not an integer".into()),
        },
    ];

    TestCase::run_result_match(tests, |t| add_values(t.0, t.1))
}

#[test]
fn test_subtract_values() {
    let tests = vec![
        TestCase {
            name: "simple",
            payload: ("1", "1"),
            criteria: Ok("0".into()),
        },
        TestCase {
            name: "large",
            payload: (
                "1000000000000000000000000",
                "100000000000000000000000000000000",
            ),
            criteria: Ok("-99999999000000000000000000000000".into()),
        },
        TestCase {
            name: "decimal",
            payload: (
                "10000000000000000000000.01",
                "100000000000000000000000000000000",
            ),
            criteria: Err("10000000000000000000000.01 is not an integer".into()),
        },
        TestCase {
            name: "negative",
            payload: ("-13213", "12332"),
            criteria: Ok("-25545".into()),
        },
        TestCase {
            name: "invalid number",
            payload: ("-13213", "hello"),
            criteria: Err("hello is not an integer".into()),
        },
    ];

    TestCase::run_result_match(tests, |t| sub_values(t.0, t.1));
}

#[test]
fn test_negative_value() {
    let tests = vec![
        TestCase {
            name: "positive number",
            payload: "100",
            criteria: Ok("-100".into()),
        },
        TestCase {
            name: "negative number",
            payload: "-100",
            criteria: Ok("100".into()),
        },
        TestCase {
            name: "decimal number",
            payload: "-100.1",
            criteria: Err("-100.1 is not an integer".into()),
        },
        TestCase {
            name: "non-number",
            payload: "hello",
            criteria: Err("hello is not an integer".into()),
        },
    ];

    TestCase::run_result_match(tests, negate_value);
}

#[test]
fn test_get_account_string() {
    let tests = vec![
        TestCase {
            name: "simple account",
            payload: AccountIdentifier {
                address: "hello".into(),
                ..Default::default()
            },
            criteria: "hello".to_string(),
        },
        TestCase {
            name: "subaccount",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            criteria: "hello:stake".to_string(),
        },
        TestCase {
            name: "subaccount with string metadata",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    metadata: [("cool".into(), "neat".into())].into(),
                }),
                ..Default::default()
            },
            criteria: "hello:stake:{\"cool\": String(\"neat\")}".to_string(),
        },
        TestCase {
            name: "subaccount with number metadata",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    metadata: [("cool".into(), 1.into())].into(),
                }),
                ..Default::default()
            },
            criteria: "hello:stake:{\"cool\": Number(1)}".to_string(),
        },
        TestCase {
            name: "subaccount with complex metadata",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    metadata: [("cool".into(), 1.into()), ("awesome".into(), "neat".into())].into(),
                }),
                ..Default::default()
            },
            criteria: "hello:stake:{\"cool\": Number(1), \"awesome\": String(\"neat\")}"
                .to_string(),
        },
    ];

    TestCase::run_output_match(tests, |t| account_string(&t));
}

#[test]
fn test_currency_string() {
    let tests = vec![
        TestCase {
            name: "simple currency",
            payload: UncheckedCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                ..Default::default()
            },
            criteria: "BTC:8".to_string(),
        },
        TestCase {
            name: "currency with string metadata",
            payload: UncheckedCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [("issuer".into(), "satoshi".into())].into(),
            },
            criteria: "BTC:8:{\"issuer\": String(\"satoshi\")}".to_string(),
        },
        TestCase {
            name: "currency with number metadata",
            payload: UncheckedCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [("issuer".into(), 1.into())].into(),
            },
            criteria: "BTC:8:{\"issuer\": Number(1)}".to_string(),
        },
        TestCase {
            name: "currency with complex metadata",
            payload: UncheckedCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [
                    ("issuer".into(), "satoshi".into()),
                    ("count".into(), 10.into()),
                ]
                .into(),
            },
            criteria: "BTC:8:{\"issuer\": String(\"satoshi\"), \"count\": Number(10)}".to_string(),
        },
    ];

    TestCase::run_output_match(tests, |t| currency_string(&t));
}

#[test]
fn test_amount_value() {
    let tests = vec![
        TestCase {
            name: "positive integer",
            payload: Some(Amount {
                value: "100".into(),
                ..Default::default()
            }),
            criteria: Ok(100.into()),
        },
        TestCase {
            name: "negative integer",
            payload: Some(Amount {
                value: "-100".into(),
                ..Default::default()
            }),
            criteria: Ok((-100).into()),
        },
        TestCase {
            name: "nil",
            payload: None,
            criteria: Err("amount value cannot be nil".to_string()),
        },
        TestCase {
            name: "float",
            payload: Some(Amount {
                value: "100.1".into(),
                ..Default::default()
            }),
            criteria: Err("100.1 is not an integer".to_string()),
        },
        TestCase {
            name: "not number",
            payload: Some(Amount {
                value: "hello".into(),
                ..Default::default()
            }),
            criteria: Err("hello is not an integer".to_string()),
        },
    ];

    TestCase::run_result_match(tests, |p| amount_value(p.as_ref()));
}

#[test]
fn test_extract_amount() {
    let currency_1 = Some(UncheckedCurrency {
        symbol: "curr1".into(),
        decimals: 4,
        ..Default::default()
    });

    let currency_2 = Some(UncheckedCurrency {
        symbol: "curr2".into(),
        decimals: 7,
        ..Default::default()
    });

    let amount1 = UncheckedAmount {
        value: "100".into(),
        currency: currency_1.clone(),
        ..Default::default()
    };

    let amount2 = UncheckedAmount {
        value: "200".into(),
        currency: currency_2.clone(),
        ..Default::default()
    };

    let balances = &[Some(amount1.clone()), Some(amount2.clone())];

    let bad_cur = Some(UncheckedCurrency {
        symbol: "no cur".into(),
        decimals: 100,
        ..Default::default()
    });

    print!("Non-existent currency: ");
    let res = extract_amount(balances, bad_cur.as_ref());
    assert_eq!(res.value, "0");
    println!("ok!");

    print!("Simple account: ");
    let res = extract_amount(balances, currency_1.as_ref());
    assert_eq!(res, amount1);
    println!("ok!");

    print!("SubAccount: ");
    let res = extract_amount(balances, currency_2.as_ref());
    assert_eq!(res, amount2);
    println!("ok!");
}
