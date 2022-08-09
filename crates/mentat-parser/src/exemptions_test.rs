use super::*;

struct FindExemptionsTest {
    account: AccountIdentifier,
    currency: Option<Currency>,
}

#[test]
fn test_find_exemptions() {
    let parser = |pe| Parser::new(None, None, pe);

    let tests = vec![
        TestCase {
            name: "no exemptions",
            payload: MethodPayload {
                caller: parser(Vec::new()),
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
                },
            },
            criteria: Vec::new(),
        },
        TestCase {
            name: "no matching exemption",
            payload: MethodPayload {
                caller: parser(vec![BalanceExemption {
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 7,
                        metadata: Default::default(),
                    }),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                }]),
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
                },
            },
            criteria: Vec::new(),
        },
        TestCase {
            name: "no matching exemptions",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
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
                },
            },
            criteria: Vec::new(),
        },
        TestCase {
            name: "currency match",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
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
                },
            },
            criteria: vec![BalanceExemption {
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                exemption_type: Some(ExemptionType::Dynamic),
                ..Default::default()
            }],
        },
        TestCase {
            name: "subaccount match",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
                payload: FindExemptionsTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        sub_account: Some(SubAccountIdentifier {
                            address: "hello".into(),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                },
            },
            criteria: vec![BalanceExemption {
                sub_account_address: Some("hello".into()),
                exemption_type: Some(ExemptionType::Dynamic),
                ..Default::default()
            }],
        },
        TestCase {
            name: "multiple match",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
                payload: FindExemptionsTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        sub_account: Some(SubAccountIdentifier {
                            address: "hello".into(),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                },
            },
            criteria: vec![
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
        },
    ];

    TestCase::run_output_match(tests, |t| {
        t.caller
            .find_exemptions(&t.payload.account, t.payload.currency.as_ref())
    });
}

struct MatchBalanceExemptionTest {
    account: AccountIdentifier,
    currency: Option<Currency>,
    difference: String,
}

#[test]
fn test_match_balance_exemption() {
    let parser = |pe| Parser::new(None, None, pe);

    let tests = vec![
        TestCase {
            name: "no exemptions",
            payload: MethodPayload {
                caller: parser(Vec::new()),
                payload: MatchBalanceExemptionTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    difference: "100".into(),
                },
            },
            criteria: None,
        },
        TestCase {
            name: "no matching exemption",
            payload: MethodPayload {
                caller: parser(vec![BalanceExemption {
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 7,
                        metadata: Default::default(),
                    }),
                    exemption_type: Some(ExemptionType::Dynamic),
                    ..Default::default()
                }]),
                payload: MatchBalanceExemptionTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                    difference: "100".into(),
                },
            },
            criteria: None,
        },
        TestCase {
            name: "no matching exemptions",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
                payload: MatchBalanceExemptionTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                    difference: "100".into(),
                },
            },
            criteria: None,
        },
        TestCase {
            name: "currency match",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
                payload: MatchBalanceExemptionTest {
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
                    difference: "100".into(),
                },
            },
            criteria: Some(BalanceExemption {
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                exemption_type: Some(ExemptionType::Dynamic),
                ..Default::default()
            }),
        },
        TestCase {
            name: "currency match, wrong sign",
            payload: MethodPayload {
                caller: parser(vec![
                    BalanceExemption {
                        currency: Some(Currency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            metadata: Default::default(),
                        }),
                        exemption_type: Some(ExemptionType::LessOrEqual),
                        ..Default::default()
                    },
                    BalanceExemption {
                        sub_account_address: Some("hello".into()),
                        exemption_type: Some(ExemptionType::Dynamic),
                        ..Default::default()
                    },
                ]),
                payload: MatchBalanceExemptionTest {
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
                    difference: "100".into(),
                },
            },
            criteria: None,
        },
        TestCase {
            name: "currency match, right sign",
            payload: MethodPayload {
                caller: parser(vec![
                    BalanceExemption {
                        currency: Some(Currency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            metadata: Default::default(),
                        }),
                        exemption_type: Some(ExemptionType::GreaterOrEqual),
                        ..Default::default()
                    },
                    BalanceExemption {
                        sub_account_address: Some("hello".into()),
                        exemption_type: Some(ExemptionType::Dynamic),
                        ..Default::default()
                    },
                ]),
                payload: MatchBalanceExemptionTest {
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
                    difference: "100".into(),
                },
            },
            criteria: Some(BalanceExemption {
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                exemption_type: Some(ExemptionType::GreaterOrEqual),
                ..Default::default()
            }),
        },
        TestCase {
            name: "currency match, zero",
            payload: MethodPayload {
                caller: parser(vec![
                    BalanceExemption {
                        currency: Some(Currency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            metadata: Default::default(),
                        }),
                        exemption_type: Some(ExemptionType::GreaterOrEqual),
                        ..Default::default()
                    },
                    BalanceExemption {
                        sub_account_address: Some("hello".into()),
                        exemption_type: Some(ExemptionType::Dynamic),
                        ..Default::default()
                    },
                ]),
                payload: MatchBalanceExemptionTest {
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
                    difference: "0".into(),
                },
            },
            criteria: Some(BalanceExemption {
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                exemption_type: Some(ExemptionType::GreaterOrEqual),
                ..Default::default()
            }),
        },
        TestCase {
            name: "subaccount match",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
                payload: MatchBalanceExemptionTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        sub_account: Some(SubAccountIdentifier {
                            address: "hello".into(),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                    difference: "100".into(),
                },
            },
            criteria: Some(BalanceExemption {
                sub_account_address: Some("hello".into()),
                exemption_type: Some(ExemptionType::Dynamic),
                ..Default::default()
            }),
        },
        TestCase {
            name: "multiple match",
            payload: MethodPayload {
                caller: parser(vec![
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
                ]),
                payload: MatchBalanceExemptionTest {
                    account: AccountIdentifier {
                        address: "test".into(),
                        sub_account: Some(SubAccountIdentifier {
                            address: "hello".into(),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    currency: Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        metadata: Default::default(),
                    }),
                    difference: "100".into(),
                },
            },
            criteria: Some(BalanceExemption {
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 8,
                    metadata: Default::default(),
                }),
                exemption_type: Some(ExemptionType::Dynamic),
                ..Default::default()
            }),
        },
    ];

    TestCase::run_output_match(tests, |t| {
        let exemptions = t
            .caller
            .find_exemptions(&t.payload.account, t.payload.currency.as_ref());
        match_balance_exemption(exemptions, &t.payload.difference)
    });
}
