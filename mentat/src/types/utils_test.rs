use std::fmt;

use indexmap::indexmap;
use serde_json::json;

use crate::{
    tests::{assert_results_correct, status_message, AsserterEqualityTest, Test},
    types::{
        account_string,
        add_values,
        amount_value,
        construct_partialblock_identifier,
        currency_string,
        extract_amount,
        hash,
        negate_value,
        sub_values,
        AccountIdentifier,
        BlockIdentifier,
        NullableAmount,
        NullableCurrency,
        PartialBlockIdentifier,
        SubAccountIdentifier,
    },
};

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

#[derive(Debug, Clone)]
struct ValuesTest {
    name: &'static str,
    a: &'static str,
    b: &'static str,
    result: Result<String, String>,
}

impl<Input> Test<Input> for ValuesTest
where
    Input: FnMut(&'static str, &'static str) -> Result<String, String>,
{
    fn run(tests: &[Self], mut func: Input) {
        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.a, test.b);
                assert_results_correct(&test.result, &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl fmt::Display for ValuesTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}

#[test]
fn test_add_values() {
    let tests = &[
        ValuesTest {
            name: "simple",
            a: "1",
            b: "1",
            result: Ok("2".into()),
        },
        ValuesTest {
            name: "large",
            a: "1000000000000000000000000",
            b: "100000000000000000000000000000000",
            result: Ok("100000001000000000000000000000000".into()),
        },
        ValuesTest {
            name: "decimal",
            a: "10000000000000000000000.01",
            b: "100000000000000000000000000000000",
            result: Err("10000000000000000000000.01 is not an integer".into()),
        },
        ValuesTest {
            name: "negative",
            a: "-13213",
            b: "12332",
            result: Ok("-881".into()),
        },
        ValuesTest {
            name: "invalid number",
            a: "-13213",
            b: "hello",
            result: Err("hello is not an integer".into()),
        },
    ];

    ValuesTest::run(tests, add_values)
}

#[test]
fn test_subtract_values() {
    let tests = &[
        ValuesTest {
            name: "simple",
            a: "1",
            b: "1",
            result: Ok("0".into()),
        },
        ValuesTest {
            name: "large",
            a: "1000000000000000000000000",
            b: "100000000000000000000000000000000",
            result: Ok("-99999999000000000000000000000000".into()),
        },
        ValuesTest {
            name: "decimal",
            a: "10000000000000000000000.01",
            b: "100000000000000000000000000000000",
            result: Err("10000000000000000000000.01 is not an integer".into()),
        },
        ValuesTest {
            name: "negative",
            a: "-13213",
            b: "12332",
            result: Ok("-25545".into()),
        },
        ValuesTest {
            name: "invalid number",
            a: "-13213",
            b: "hello",
            result: Err("hello is not an integer".into()),
        },
    ];

    ValuesTest::run(tests, sub_values);
}

#[test]
fn test_negative_value() {
    let tests = &[
        ValuesTest {
            name: "positive number",
            a: "100",
            b: "",
            result: Ok("-100".into()),
        },
        ValuesTest {
            name: "negative number",
            a: "-100",
            b: "",
            result: Ok("100".into()),
        },
        ValuesTest {
            name: "decimal number",
            a: "-100.1",
            b: "",
            result: Err("-100.1 is not an integer".into()),
        },
        ValuesTest {
            name: "non-number",
            a: "hello",
            b: "",
            result: Err("hello is not an integer".into()),
        },
    ];

    ValuesTest::run(tests, |a, _| negate_value(a));
}

#[test]
fn test_get_account_string() {
    let tests = &[
        AsserterEqualityTest {
            name: "simple account",
            payload: AccountIdentifier {
                address: "hello".into(),
                ..Default::default()
            },
            res: "hello".to_string(),
        },
        AsserterEqualityTest {
            name: "subaccount",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            res: "hello:stake".to_string(),
        },
        AsserterEqualityTest {
            name: "subaccount with string metadata",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    metadata: [("cool".into(), "neat".into())].into(),
                }),
                ..Default::default()
            },
            res: "hello:stake:{\"cool\": String(\"neat\")}".to_string(),
        },
        AsserterEqualityTest {
            name: "subaccount with number metadata",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    metadata: [("cool".into(), 1.into())].into(),
                }),
                ..Default::default()
            },
            res: "hello:stake:{\"cool\": Number(1)}".to_string(),
        },
        AsserterEqualityTest {
            name: "subaccount with complex metadata",
            payload: AccountIdentifier {
                address: "hello".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "stake".into(),
                    metadata: [("cool".into(), 1.into()), ("awesome".into(), "neat".into())].into(),
                }),
                ..Default::default()
            },
            res: "hello:stake:{\"cool\": Number(1), \"awesome\": String(\"neat\")}".to_string(),
        },
    ];

    AsserterEqualityTest::run(tests, account_string);
}

#[test]
fn test_currency_string() {
    let tests = &[
        AsserterEqualityTest {
            name: "simple currency",
            payload: NullableCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                ..Default::default()
            },
            res: "BTC:8".to_string(),
        },
        AsserterEqualityTest {
            name: "currency with string metadata",
            payload: NullableCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [("issuer".into(), "satoshi".into())].into(),
            },
            res: "BTC:8:{\"issuer\": String(\"satoshi\")}".to_string(),
        },
        AsserterEqualityTest {
            name: "currency with number metadata",
            payload: NullableCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [("issuer".into(), 1.into())].into(),
            },
            res: "BTC:8:{\"issuer\": Number(1)}".to_string(),
        },
        AsserterEqualityTest {
            name: "currency with complex metadata",
            payload: NullableCurrency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [
                    ("issuer".into(), "satoshi".into()),
                    ("count".into(), 10.into()),
                ]
                .into(),
            },
            res: "BTC:8:{\"issuer\": String(\"satoshi\"), \"count\": Number(10)}".to_string(),
        },
    ];

    AsserterEqualityTest::run(tests, currency_string);
}

#[test]
fn test_amount_value() {
    let tests = &[
        AsserterEqualityTest {
            name: "positive integer",
            payload: Some(NullableAmount {
                value: "100".into(),
                ..Default::default()
            }),
            res: Ok(100.into()),
        },
        AsserterEqualityTest {
            name: "negative integer",
            payload: Some(NullableAmount {
                value: "-100".into(),
                ..Default::default()
            }),
            res: Ok((-100).into()),
        },
        AsserterEqualityTest {
            name: "nil",
            payload: None,
            res: Err("amount value cannot be nil".to_string()),
        },
        AsserterEqualityTest {
            name: "float",
            payload: Some(NullableAmount {
                value: "100.1".into(),
                ..Default::default()
            }),
            res: Err("100.1 is not an integer".to_string()),
        },
        AsserterEqualityTest {
            name: "not number",
            payload: Some(NullableAmount {
                value: "hello".into(),
                ..Default::default()
            }),
            res: Err("hello is not an integer".to_string()),
        },
    ];

    AsserterEqualityTest::run(tests, |p| amount_value(p.as_ref()));
}

#[test]
fn test_extract_amount() {
    let currency_1 = Some(NullableCurrency {
        symbol: "curr1".into(),
        decimals: 4,
        ..Default::default()
    });

    let currency_2 = Some(NullableCurrency {
        symbol: "curr2".into(),
        decimals: 7,
        ..Default::default()
    });

    let amount1 = NullableAmount {
        value: "100".into(),
        currency: currency_1.clone(),
        ..Default::default()
    };

    let amount2 = NullableAmount {
        value: "200".into(),
        currency: currency_2.clone(),
        ..Default::default()
    };

    let balances = &[Some(amount1.clone()), Some(amount2.clone())];

    let bad_cur = Some(NullableCurrency {
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
