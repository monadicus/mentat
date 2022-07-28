use crate::{
    asserter::tests::AsserterEqualityTest,
    types::{
        account_string, add_values, amount_value, construct_partialblock_identifier,
        currency_string, extract_amount, negate_value, sub_values, AccountIdentifier, Amount,
        BlockIdentifier, Currency, PartialBlockIdentifier, SubAccountIdentifier,
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
    todo!()
}

#[derive(Debug, Clone)]
struct ValuesTest {
    name: &'static str,
    a: &'static str,
    b: &'static str,
    result: Result<String, String>,
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

    for test in tests {
        print!("{}: ", test.name);
        let res = add_values(test.a, test.b);
        assert_eq!(res, test.result);
        println!("ok!");
    }
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

    for test in tests {
        print!("{}: ", test.name);
        let res = sub_values(test.a, test.b);
        assert_eq!(res, test.result);
        println!("ok!");
    }
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

    for test in tests {
        print!("{}: ", test.name);
        let res = negate_value(test.a);
        assert_eq!(res, test.result);
        println!("ok!");
    }
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
            res: "hello",
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
            res: "hello:stake",
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
            res: "hello:stake:map[cool:neat]",
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
            res: "hello:stake:map[cool:1]",
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
            res: "hello:stake:map[awesome:neat cool:1]",
        },
    ];

    for test in tests {
        print!("{}: ", test.name);
        let res = account_string(&test.payload);
        assert_eq!(res, test.res);
        println!("ok!");
    }
}

#[test]
fn test_currency_string() {
    let tests = &[
        AsserterEqualityTest {
            name: "simple currency",
            payload: Currency {
                symbol: "BTC".into(),
                decimals: 8,
                ..Default::default()
            },
            res: "BTC:8",
        },
        AsserterEqualityTest {
            name: "currency with string metadata",
            payload: Currency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [("issuer".into(), "satoshi".into())].into(),
            },
            res: "BTC:8:map[issuer:satoshi]",
        },
        AsserterEqualityTest {
            name: "currency with number metadata",
            payload: Currency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [("issuer".into(), 1.into())].into(),
            },
            res: "BTC:8:map[issuer:1]",
        },
        AsserterEqualityTest {
            name: "currency with complex metadata",
            payload: Currency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: [
                    ("issuer".into(), "satoshi".into()),
                    ("count".into(), 10.into()),
                ]
                .into(),
            },
            res: "BTC:8:map[count:10 issuer:satoshi]",
        },
    ];

    for test in tests {
        print!("{}: ", test.name);
        let res = currency_string(&test.payload);
        assert_eq!(res, test.res);
        println!("ok!");
    }
}

#[test]
fn test_marshal_map() {
    todo!()
}

#[test]
fn test_unmarshal_map() {
    todo!()
}

#[test]
fn test_amount_value() {
    let tests = &[
        AsserterEqualityTest {
            name: "positive integer",
            payload: Amount {
                value: "100".into(),
                ..Default::default()
            },
            res: Ok(100.into()),
        },
        AsserterEqualityTest {
            name: "negative integer",
            payload: Amount {
                value: "-100".into(),
                ..Default::default()
            },
            res: Ok((-100).into()),
        },
        AsserterEqualityTest {
            name: "nil",
            payload: Amount {
                value: "".into(),
                ..Default::default()
            },
            res: Err("amount value cannot be nil".to_string()),
        },
        AsserterEqualityTest {
            name: "float",
            payload: Amount {
                value: "100.1".into(),
                ..Default::default()
            },
            res: Err("100.1 is not an integer".to_string()),
        },
        AsserterEqualityTest {
            name: "not number",
            payload: Amount {
                value: "hello".into(),
                ..Default::default()
            },
            res: Err("hello is not an integer".to_string()),
        },
    ];

    for test in tests {
        print!("{}: ", test.name);
        let res = amount_value(&test.payload);
        assert_eq!(res, test.res);
        println!("ok!");
    }
}

#[test]
fn test_extract_amount() {
    let currency_1 = Currency {
        symbol: "curr1".into(),
        decimals: 4,
        ..Default::default()
    };

    let currency_2 = Currency {
        symbol: "curr2".into(),
        decimals: 7,
        ..Default::default()
    };

    let amount1 = Amount {
        value: "100".into(),
        currency: Some(currency_1.clone()),
        ..Default::default()
    };

    let amount2 = Amount {
        value: "200".into(),
        currency: Some(currency_2.clone()),
        ..Default::default()
    };

    let balances = &[Some(amount1.clone()), Some(amount2.clone())];

    let bad_cur = Currency {
        symbol: "no cur".into(),
        decimals: 100,
        ..Default::default()
    };

    print!("Non-existent currency: ");
    let res = extract_amount(balances, &bad_cur);
    assert_eq!(res.value, "0");
    println!("ok!");

    print!("Simple account: ");
    let res = extract_amount(balances, &currency_1);
    assert_eq!(res, amount1);
    println!("ok!");

    print!("SubAccount: ");
    let res = extract_amount(balances, &currency_2);
    assert_eq!(res, amount2);
    println!("ok!");
}
