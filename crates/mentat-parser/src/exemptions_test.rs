use super::*;

struct FindExemptionsTest {
    account: AccountIdentifier,
    currency: Option<Currency>,
    expected: Vec<BalanceExemption>,
}

#[test]
fn test_find_exemptions() {
    let tests = vec![
        CustomParserTest {
            name: "no exemptions",
            payload: FindExemptionsTest {
                account: AccountIdentifier {
                    address: "test".into(),
                    ..Default::default()
                },
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                expected: Vec::new(),
            },
            asserter_extras: (),
            parser_extras: Vec::new(),
            err: None,
        },
        CustomParserTest {
            name: "no matching exemption",
            payload: FindExemptionsTest {
                account: AccountIdentifier {
                    address: "test".into(),
                    ..Default::default()
                },
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                expected: Vec::new(),
            },
            asserter_extras: (),
            parser_extras: vec![BalanceExemption {
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 7,
                    metadata: Default::default(),
                }),
                exemption_type: Some(ExemptionType::Dynamic),
                ..Default::default()
            }],
            err: None,
        },
        CustomParserTest {
            name: "no matching exemptions",
            payload: FindExemptionsTest {
                account: AccountIdentifier {
                    address: "test".into(),
                    ..Default::default()
                },
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                expected: Vec::new(),
            },
            asserter_extras: (),
            parser_extras: vec![
                BalanceExemption {
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 7,
                        metadata: Default::default(),
                    }),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                },
                BalanceExemption {
                    sub_account_address: Some("hello".into()),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                },
            ],
            err: None,
        },
        CustomParserTest {
            name: "currency match",
            payload: FindExemptionsTest {
                account: AccountIdentifier {
                    address: "test".into(),
                    sub_account: Some(SubAccountIdentifier {
                        address: "blah".into(),
                        metadata: Default::default(),
                    }),
                    ..Default::default()
                },
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                expected: vec![BalanceExemption {
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                }],
            },
            asserter_extras: (),
            parser_extras: vec![
                BalanceExemption {
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                },
                BalanceExemption {
                    sub_account_address: Some("hello".into()),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                },
            ],
            err: None,
        },
    ];

    CustomParserTest::run(
        tests,
        |_| None,
        |a, pe| Parser::new(a, None, pe),
        |parser, payload| {
            let res = parser.find_exemptions(&payload.account, payload.currency.as_ref());

            if res != payload.expected {
                println!(
                    "test returned wrong value: `{:?}` != `{:?}`",
                    payload.expected, res
                );
                false
            } else {
                println!("ok!");
                true
            }
        },
    );
}
