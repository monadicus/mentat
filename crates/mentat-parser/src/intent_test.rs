use super::{balance_changes_test::simple_asserter_configuration, *};

#[test]
fn test_expected_operation() {
    let tests = vec![
        TestCase {
            name: "simple match",
            payload: (
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        ..Default::default()
                    },
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 3,
                        ..Default::default()
                    },
                    related_operations: vec![OperationIdentifier {
                        index: 2,
                        ..Default::default()
                    }],
                    status: Some("success".into()),
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            result: false,
        },
        TestCase {
            name: "account mismatch",
            payload: (
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        ..Default::default()
                    },
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 3,
                        ..Default::default()
                    },
                    related_operations: vec![OperationIdentifier {
                        index: 2,
                        ..Default::default()
                    }],
                    status: Some("success".into()),
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            result: true,
        },
        TestCase {
            name: "amount mismatch",
            payload: (
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        ..Default::default()
                    },
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 3,
                        ..Default::default()
                    },
                    related_operations: vec![OperationIdentifier {
                        index: 2,
                        ..Default::default()
                    }],
                    status: Some("success".into()),
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "150".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            result: true,
        },
        TestCase {
            name: "type mismatch",
            payload: (
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        ..Default::default()
                    },
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier {
                        index: 3,
                        ..Default::default()
                    },
                    related_operations: vec![OperationIdentifier {
                        index: 2,
                        ..Default::default()
                    }],
                    status: Some("success".into()),
                    type_: "reward".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            result: true,
        },
    ];

    TestCase::run_is_err(tests, |t| expected_operation(&t.0, &t.1));
}

#[derive(Default)]
struct ExpectedOperationTest {
    intent: Vec<Operation>,
    observed: Vec<Operation>,
    err_extra: bool,
    confirm_success: bool,
}

#[test]
fn test_expected_operations() {
    let tests = vec![
        TestCase {
            name: "simple match",
            payload: ExpectedOperationTest {
                intent: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        ..Default::default()
                    },
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                observed: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 3,
                        ..Default::default()
                    },
                    related_operations: vec![OperationIdentifier {
                        index: 2,
                        ..Default::default()
                    }],
                    status: Some("success".into()),
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            },
            result: false,
        },
        TestCase {
            name: "simple unbroadcast match",
            payload: ExpectedOperationTest {
                intent: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            ..Default::default()
                        },
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 5,
                            ..Default::default()
                        },
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                observed: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        },
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 3,
                            ..Default::default()
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        }],
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            result: false,
        },
        TestCase {
            name: "simple match (confirm success)",
            payload: ExpectedOperationTest {
                intent: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            ..Default::default()
                        },
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 5,
                            ..Default::default()
                        },
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                observed: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        },
                        status: Some("success".into()),
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 3,
                            ..Default::default()
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        }],
                        status: Some("success".into()),
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                confirm_success: true,
                ..Default::default()
            },
            result: false,
        },
        TestCase {
            name: "simple match (confirm success) errors",
            payload: ExpectedOperationTest {
                intent: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            ..Default::default()
                        },
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 5,
                            ..Default::default()
                        },
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                observed: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        },
                        status: Some("success".into()),
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 3,
                            ..Default::default()
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        }],
                        status: Some("failure".into()),
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                confirm_success: true,
                ..Default::default()
            },
            result: true,
        },
        TestCase {
            name: "errors extra",
            payload: ExpectedOperationTest {
                intent: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        ..Default::default()
                    },
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                observed: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        },
                        status: Some("success".into()),
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 3,
                            ..Default::default()
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 2,
                            ..Default::default()
                        }],
                        status: Some("success".into()),
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                err_extra: true,
                ..Default::default()
            },
            result: true,
        },
        TestCase {
            name: "missing match",
            payload: ExpectedOperationTest {
                intent: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            ..Default::default()
                        },
                        type_: "transfer".into(),
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 5,
                            ..Default::default()
                        },
                        type_: "fee".into(),
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        amount: Some(Amount {
                            value: "50".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                observed: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 3,
                        ..Default::default()
                    },
                    related_operations: vec![OperationIdentifier {
                        index: 2,
                        ..Default::default()
                    }],
                    status: Some("success".into()),
                    type_: "transfer".into(),
                    account: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    amount: Some(Amount {
                        value: "100".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            },
            result: true,
        },
    ];

    let asserter = simple_asserter_configuration(vec![
        Some(OperationStatus {
            status: "success".into(),
            successful: true,
        }),
        Some(OperationStatus {
            status: "failure".into(),
            successful: false,
        }),
    ]);
    let parser = Parser::new(asserter, None, Vec::new());

    TestCase::run_is_err(tests, |t| {
        parser.expected_operations(&t.intent, &t.observed, t.err_extra, t.confirm_success)
    })
}

#[test]
fn test_expected_signers() {
    let tests = vec![
        TestCase {
            name: "simple match",
            payload: (
                vec![
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                vec![
                    AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    },
                    AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    },
                ],
            ),
            result: false,
        },
        TestCase {
            name: "complex match",
            payload: (
                vec![
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr1".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "test".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                vec![
                    AccountIdentifier {
                        address: "addr1".into(),
                        sub_account: Some(SubAccountIdentifier {
                            address: "test".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    },
                ],
            ),
            result: false,
        },
        TestCase {
            name: "missing observed signer",
            payload: (
                vec![
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                vec![AccountIdentifier {
                    address: "addr1".into(),
                    ..Default::default()
                }],
            ),
            result: true,
        },
        TestCase {
            name: "complex mismatch",
            payload: (
                vec![
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    SigningPayload {
                        account_identifier: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                vec![
                    AccountIdentifier {
                        address: "addr1".into(),
                        sub_account: Some(SubAccountIdentifier {
                            address: "test".into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    },
                ],
            ),
            result: true,
        },
        TestCase {
            name: "extra observed signer",
            payload: (
                vec![SigningPayload {
                    account_identifier: Some(AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                vec![
                    AccountIdentifier {
                        address: "addr1".into(),
                        ..Default::default()
                    },
                    AccountIdentifier {
                        address: "addr2".into(),
                        ..Default::default()
                    },
                ],
            ),
            result: true,
        },
    ];

    TestCase::run_is_err(tests, |t| expected_signers(&t.0, &t.1))
}
