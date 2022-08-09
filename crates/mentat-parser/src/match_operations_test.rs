use num_bigint_dig::BigInt;

use super::*;

#[derive(Default, PartialEq, Eq)]
struct MatchOperationsTest {
    operations: Vec<Option<Operation>>,
    descriptions: Descriptions,
}

#[test]
fn test_match_operations() {
    let tests = vec![TestCase {
        name: "simple transfer (with extra op)",
        payload: MatchOperationsTest {
            operations: vec![
                Some(Operation {
                    account: Some(AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                // extra op ignored
                Some(Operation::default()),
                Some(Operation {
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "-100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            ],
            descriptions: Descriptions {
                opposite_amounts: vec![vec![0, 1]],
                operation_descriptions: vec![
                    Some(OperationDescription {
                        account: Some(AccountDescription {
                            exists: true,
                            ..Default::default()
                        }),
                        amount: Some(AmountDescription {
                            exists: true,
                            sign: AmountSign::NEGATIVE,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    Some(OperationDescription {
                        account: Some(AccountDescription {
                            exists: true,
                            ..Default::default()
                        }),
                        amount: Some(AmountDescription {
                            exists: true,
                            sign: AmountSign::POSITIVE,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            },
        },
        result: Some(vec![
            Some(Match {
                operations: vec![Some(Operation {
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "-100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })],
                amounts: vec![Some(BigInt::from(-100))],
            }),
            Some(Match {
                operations: vec![Some(Operation {
                    account: Some(AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })],
                amounts: vec![Some(BigInt::from(100))],
            }),
        ]),
    }];

    TestCase::run_ok_match(tests, |payload| {
        match_operations(payload.descriptions, payload.operations)
    });
}

#[derive(Debug, Default, PartialEq, Eq)]
struct MatchTestResult {
    op: Option<Operation>,
    amt: Option<BigInt>,
}

impl From<(Option<&Operation>, Option<&BigInt>)> for MatchTestResult {
    fn from((op, amt): (Option<&Operation>, Option<&BigInt>)) -> Self {
        Self {
            op: op.cloned(),
            amt: amt.cloned(),
        }
    }
}

#[test]
fn test_match() {
    let tests = vec![
        TestCase {
            name: "nil match",
            payload: None,
            result: MatchTestResult::default(),
        },
        TestCase {
            name: "empty match",
            payload: Some(Match::default()),
            result: MatchTestResult::default(),
        },
        TestCase {
            name: "single op match",
            payload: Some(Match {
                operations: vec![Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                })],
                amounts: vec![Some(BigInt::from(100))],
            }),
            result: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                }),
                amt: Some(BigInt::from(100)),
            },
        },
        TestCase {
            name: "multi-op match",
            payload: Some(Match {
                operations: vec![
                    Some(Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        ..Default::default()
                    }),
                ],
                amounts: vec![Some(BigInt::from(100)), Some(BigInt::from(200))],
            }),
            result: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                }),
                amt: Some(BigInt::from(100)),
            },
        },
        TestCase {
            name: "single op match with nil amount",
            payload: Some(Match {
                operations: vec![Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                })],
                amounts: vec![None],
            }),
            result: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                }),
                amt: None,
            },
        },
    ];

    TestCase::run_output_match(tests, |test| Match::first(test.as_ref()).into());
}
