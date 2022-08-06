use super::*;

#[test]
fn test_expected_operation() {
    let tests = vec![
        FnTest {
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
        FnTest {
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
        FnTest {
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
        FnTest {
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

    FnTest::run_is_err(tests, |t| expected_operation(&t.0, &t.1));
}

#[test]
fn test_expected_operations() {
    todo!()
}

#[test]
fn test_expected_signers() {
    let tests = vec![
        FnTest {
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
        FnTest {
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
        FnTest {
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
        FnTest {
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
        FnTest {
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

    FnTest::run_is_err(tests, |t| expected_signers(&t.0, &t.1))
}
