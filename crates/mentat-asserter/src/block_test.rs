use std::path::{Path, PathBuf};

use super::*;

#[test]
fn test_block_identifier() {
    let tests = vec![
        TestCase {
            name: "valid identifier",
            payload: Some(NullableBlockIdentifier {
                index: 1,
                hash: "block 1".into(),
            }),
            criteria: None,
        },
        TestCase {
            name: "nil identifier",
            payload: None,
            criteria: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        TestCase {
            name: "invalid index",
            payload: Some(NullableBlockIdentifier {
                index: -1,
                hash: "block 1".into(),
            }),
            criteria: Some(BlockError::BlockIdentifierIndexIsNeg.into()),
        },
        TestCase {
            name: "invalid hash",
            payload: Some(NullableBlockIdentifier {
                index: 1,
                hash: String::new(),
            }),
            criteria: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| block_identifier(t.as_ref()));
}

#[test]
fn test_amount() {
    let tests = vec![
        TestCase {
            name: "valid amount",
            payload: Some(NullableAmount {
                value: "100000".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: None,
        },
        TestCase {
            name: "valid amount no decimals",
            payload: Some(NullableAmount {
                value: "100000".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: Default::default(),
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: None,
        },
        TestCase {
            name: "valid negative amount",
            payload: Some(NullableAmount {
                value: "-100000".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: None,
        },
        TestCase {
            name: "nil amount",
            payload: None,
            criteria: Some(BlockError::AmountValueMissing.into()),
        },
        TestCase {
            name: "nil currency",
            payload: Some(NullableAmount {
                value: "-100000".into(),
                currency: None,
                metadata: Default::default(),
            }),
            criteria: Some(BlockError::AmountCurrencyIsNil.into()),
        },
        TestCase {
            name: "invalid non number",
            payload: Some(NullableAmount {
                value: "blah".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: Some(AsserterError::from(format!(
                "{}: blah",
                BlockError::AmountIsNotInt
            ))),
        },
        TestCase {
            name: "invalid integer format",
            payload: Some(NullableAmount {
                value: "1.0".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: Some(AsserterError::from(format!(
                "{}: 1.0",
                BlockError::AmountIsNotInt
            ))),
        },
        TestCase {
            name: "invalid non-integer",
            payload: Some(NullableAmount {
                value: "1.1".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: Some(AsserterError::from(format!(
                "{}: 1.1",
                BlockError::AmountIsNotInt
            ))),
        },
        TestCase {
            name: "invalid symbol",
            payload: Some(NullableAmount {
                value: "11".into(),
                currency: Some(NullableCurrency {
                    symbol: String::new(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: Some(BlockError::AmountCurrencySymbolEmpty.into()),
        },
        TestCase {
            name: "invalid decimals",
            payload: Some(NullableAmount {
                value: "111".into(),
                currency: Some(NullableCurrency {
                    symbol: "BTC".into(),
                    decimals: -1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: Some(BlockError::AmountCurrencyHasNegDecimals.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| amount(t.as_ref()));
}

#[derive(Default)]
struct OperationIdentTest {
    ident: Option<NullableOperationIdentifier>,
    index: isize,
}

impl OperationIdentTest {
    fn run(&self) -> AssertResult<()> {
        operation_identifier(self.ident.as_ref(), self.index)
    }
}

#[test]
fn test_operation_identifier() {
    let valid_network_index = 1;
    let invalid_network_index = -1;

    let tests = vec![
        TestCase {
            name: "valid identifier",
            payload: Some(OperationIdentTest {
                ident: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                index: 0,
            }),
            criteria: None,
        },
        TestCase {
            name: "nil identifier",
            payload: Some(OperationIdentTest {
                ident: None,
                index: 0,
            }),
            criteria: Some(BlockError::OperationIdentifierIndexIsNil.into()),
        },
        TestCase {
            name: "out-of-order index",
            payload: Some(OperationIdentTest {
                ident: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                index: 1,
            }),
            criteria: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        TestCase {
            name: "valid identifier with network index",
            payload: Some(OperationIdentTest {
                ident: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: Some(valid_network_index),
                }),
                index: 0,
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid identifier with network index",
            payload: Some(OperationIdentTest {
                ident: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: Some(invalid_network_index),
                }),
                index: 0,
            }),
            criteria: Some(BlockError::OperationIdentifierNetworkIndexInvalid.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| t.unwrap().run());
}

#[test]
fn test_account_identifier() {
    let tests = vec![
        TestCase {
            name: "valid identifier",
            payload: Some(AccountIdentifier {
                address: "acct1".into(),
                sub_account: None,
                metadata: Default::default(),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid identifier",
            payload: Some(AccountIdentifier {
                address: Default::default(),
                sub_account: None,
                metadata: Default::default(),
            }),
            criteria: Some(BlockError::AccountAddrMissing.into()),
        },
        TestCase {
            name: "valid identifier with subaccount",
            payload: Some(AccountIdentifier {
                address: "acct1".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "acct2".into(),
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid identifier with subaccount",
            payload: Some(AccountIdentifier {
                address: "acct1".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: String::new(),
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            criteria: Some(BlockError::AccountSubAccountAddrMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| account_identifier(t.as_ref()));
}

#[derive(Default)]
struct OperationValidationsTest {
    operations: Vec<Option<NullableOperation>>,
    construction: bool,
}

#[test]
fn test_operations_validations() {
    let valid_deposit_amt = Some(NullableAmount {
        value: "1000".into(),
        currency: Some(NullableCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_withdraw_amt = Some(NullableAmount {
        value: "-1000".into(),
        currency: Some(NullableCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_fee_amt = Some(NullableAmount {
        value: "-100".into(),
        currency: Some(NullableCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let invalid_fee_amt = Some(NullableAmount {
        value: "100".into(),
        currency: Some(NullableCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_account = Some(AccountIdentifier {
        address: "test".into(),
        ..Default::default()
    });

    let asserter = |validation_file_path: &Path| {
        Asserter::new_client_with_responses(
            Some(NetworkIdentifier {
                blockchain: "HELLO".into(),
                network: "WORLD".into(),
                sub_network_identifier: None,
            }),
            Some(NullableNetworkStatusResponse {
                current_block_identifier: Some(NullableBlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: Some(NullableBlockIdentifier {
                    index: 0,
                    hash: "block 0".into(),
                }),
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Some(Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                })],
            }),
            Some(NullableNetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(NullableAllow {
                    operation_statuses: vec![
                        Some(OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        }),
                        Some(OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        }),
                    ],
                    operation_types: vec!["PAYMENT".into(), "FEE".into()],
                    ..Default::default()
                }),
            }),
            Some(&validation_file_path.to_path_buf()),
        )
        .unwrap()
    };

    let tests = vec![
        TestCase {
            name: "valid operations based on validation file",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_balanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_withdraw_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 2,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_fee_amt.clone(),
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: None,
        },
        TestCase {
            name: "throw error on missing fee operation",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_balanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_withdraw_amt,
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: Some(BlockError::FeeCountMismatch.into()),
        },
        TestCase {
            name: "throw error on missing payment operation",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_balanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_fee_amt.clone(),
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: Some(BlockError::PaymentCountMismatch.into()),
        },
        TestCase {
            name: "throw error on payment amount not balancing",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_balanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: Some(NullableAmount {
                                value: "-2000".into(),
                                currency: Some(NullableCurrency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    metadata: Default::default(),
                                }),
                                metadata: Default::default(),
                            }),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 2,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_fee_amt.clone(),
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: Some(BlockError::PaymentAmountNotBalancing.into()),
        },
        TestCase {
            name: "valid operations based on validation file - unbalanced",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_unbalanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: Some(NullableAmount {
                                value: "-2000".into(),
                                currency: Some(NullableCurrency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    metadata: Default::default(),
                                }),
                                metadata: Default::default(),
                            }),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 2,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_fee_amt.clone(),
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: None,
        },
        TestCase {
            name: "fee operation shouldn't contain related_operation key",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_unbalanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: Some(NullableAmount {
                                value: "-2000".into(),
                                currency: Some(NullableCurrency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    metadata: Default::default(),
                                }),
                                metadata: Default::default(),
                            }),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 2,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_fee_amt.clone(),
                            related_operations: vec![Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            })],
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: Some(BlockError::RelatedOperationInFeeNotAllowed.into()),
        },
        TestCase {
            name: "fee amount is non-negative",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_unbalanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt.clone(),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: Some(NullableAmount {
                                value: "-2000".into(),
                                currency: Some(NullableCurrency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    metadata: Default::default(),
                                }),
                                metadata: Default::default(),
                            }),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 2,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: invalid_fee_amt,
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: Some(BlockError::FeeAmountNotNegative.into()),
        },
        TestCase {
            name: "fee amount is negative as expected",
            payload: MethodPayload {
                caller: asserter(Path::new("validation_fee_and_payment_unbalanced.json")),
                payload: OperationValidationsTest {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 0,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: valid_deposit_amt,
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 1,
                                network_index: None,
                            }),
                            type_: "PAYMENT".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account.clone(),
                            amount: Some(NullableAmount {
                                value: "-2000".into(),
                                currency: Some(NullableCurrency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    metadata: Default::default(),
                                }),
                                metadata: Default::default(),
                            }),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(NullableOperationIdentifier {
                                index: 2,
                                network_index: None,
                            }),
                            type_: "FEE".into(),
                            status: Some("SUCCESS".into()),
                            account: valid_account,
                            amount: valid_fee_amt,
                            ..Default::default()
                        }),
                    ],
                    construction: false,
                },
            },
            criteria: None,
        },
    ];

    TestCase::run_err_match(tests, |t| {
        t.caller
            .operations(&t.payload.operations, t.payload.construction)
    });
}

#[derive(Default)]
struct OperationTest {
    operation: Option<NullableOperation>,
    index: isize,
    successful: bool,
    construction: bool,
}

#[test]
fn test_operation() {
    let valid_amount = Some(NullableAmount {
        value: "1000".into(),
        currency: Some(NullableCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_account = Some(AccountIdentifier {
        address: "test".into(),
        ..Default::default()
    });

    let tests = vec![
        TestCase {
            name: "valid operation",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    account: valid_account.clone(),
                    amount: valid_amount.clone(),
                    ..Default::default()
                }),
                index: 1,
                successful: true,
                construction: false,
            },
            criteria: None,
        },
        TestCase {
            name: "valid operation no account",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    ..Default::default()
                }),
                index: 1,
                successful: true,
                construction: false,
            },
            criteria: None,
        },
        TestCase {
            name: "nil operation",
            payload: OperationTest {
                operation: None,
                index: 1,
                successful: false,
                construction: false,
            },
            criteria: Some(BlockError::OperationIsNil.into()),
        },
        TestCase {
            name: "invalid operation no account",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    amount: valid_amount.clone(),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: false,
            },
            criteria: Some(BlockError::AccountIsNil.into()),
        },
        TestCase {
            name: "invalid operation empty account",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    account: Some(AccountIdentifier::default()),
                    amount: valid_amount.clone(),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: false,
            },
            criteria: Some(BlockError::AccountAddrMissing.into()),
        },
        TestCase {
            name: "invalid operation invalid index",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    ..Default::default()
                }),
                index: 2,
                successful: false,
                construction: false,
            },
            criteria: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        TestCase {
            name: "invalid operation invalid type",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "STAKE".into(),
                    status: Some("SUCCESS".into()),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: false,
            },
            criteria: Some(BlockError::OperationTypeInvalid.into()),
        },
        TestCase {
            name: "unsuccessful operation",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("FAILURE".into()),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: false,
            },
            criteria: None,
        },
        TestCase {
            name: "invalid operation invalid status",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("DEFERRED".into()),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: false,
            },
            criteria: Some(BlockError::OperationStatusInvalid.into()),
        },
        TestCase {
            name: "valid construction operation",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    account: valid_account.clone(),
                    amount: valid_amount.clone(),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: true,
            },
            criteria: None,
        },
        TestCase {
            name: "valid construction operation (empty status)",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some(String::new()),
                    account: valid_account.clone(),
                    amount: valid_amount.clone(),
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: true,
            },
            criteria: None,
        },
        TestCase {
            name: "invalid construction operation",
            payload: OperationTest {
                operation: Some(NullableOperation {
                    operation_identifier: Some(NullableOperationIdentifier {
                        index: 1,
                        network_index: None,
                    }),
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    account: valid_account,
                    amount: valid_amount,
                    ..Default::default()
                }),
                index: 1,
                successful: false,
                construction: true,
            },
            criteria: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
    ];

    let asserter = Asserter::new_client_with_responses(
        Some(NetworkIdentifier {
            blockchain: "HELLO".into(),
            network: "WORLD".into(),
            sub_network_identifier: None,
        }),
        Some(NullableNetworkStatusResponse {
            current_block_identifier: Some(NullableBlockIdentifier {
                index: 100,
                hash: "block 100".into(),
            }),
            current_block_timestamp: MIN_UNIX_EPOCH + 1,
            genesis_block_identifier: Some(NullableBlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            }),
            oldest_block_identifier: None,
            sync_status: None,
            peers: vec![Some(Peer {
                peer_id: "peer 1".into(),
                metadata: Default::default(),
            })],
        }),
        Some(NullableNetworkOptionsResponse {
            version: Some(Version {
                rosetta_version: "1.4.0".into(),
                node_version: "1.0".into(),
                middleware_version: None,
                metadata: Default::default(),
            }),
            allow: Some(NullableAllow {
                operation_statuses: vec![
                    Some(OperationStatus {
                        status: "SUCCESS".into(),
                        successful: true,
                    }),
                    Some(OperationStatus {
                        status: "FAILURE".into(),
                        successful: false,
                    }),
                ],
                operation_types: vec!["PAYMENT".into()],
                ..Default::default()
            }),
        }),
        None,
    )
    .unwrap();

    let test_count = tests.len();
    let failed = tests
        .into_iter()
        .map(|test| {
            println!("{}: ", test.name);

            print!("Testing operation: ");
            let payload = test.payload;
            if !check_err_match(
                &test.criteria,
                &asserter.operation(
                    payload.operation.as_ref(),
                    payload.index,
                    payload.construction,
                ),
            ) {
                return false;
            }

            if test.criteria.is_none() && !payload.construction {
                let op = payload.operation.clone().unwrap().into();
                print!("Testing operation successful: ");
                let successful = asserter.operation_successful(&op).unwrap();

                if payload.successful == successful {
                    println!("ok!");
                } else {
                    println!("failed!");
                }
                return payload.successful == successful;
            }

            true
        })
        .filter(|t| !t)
        .count();
    status_message(failed, test_count);
}

#[derive(Default)]
struct BlockTestExtras {
    genesis_index: isize,
    start_index: Option<isize>,
    validation_file_path: Option<PathBuf>,
}

#[test]
fn test_block() {
    let genesis_ident = NullableBlockIdentifier {
        hash: "gen".into(),
        index: 0,
    };
    let valid_block_ident = NullableBlockIdentifier {
        hash: "blah".into(),
        index: 100,
    };
    let valid_parent_block_ident = NullableBlockIdentifier {
        hash: "blah parent".into(),
        index: 99,
    };
    let valid_amount = Some(NullableAmount {
        value: "1000".into(),
        currency: Some(NullableCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_account = Some(AccountIdentifier {
        address: "test".into(),
        ..Default::default()
    });
    let valid_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
        ],
        related_transactions: vec![Some(NullableRelatedTransaction {
            network_identifier: Some(NetworkIdentifier {
                blockchain: "HELLO".into(),
                network: "WORLD".into(),
                sub_network_identifier: None,
            }),
            transaction_identifier: Some(TransactionIdentifier {
                hash: "blah".into(),
            }),
            direction: NullableDirection::FORWARD.into(),
        })],
        metadata: Default::default(),
    });
    let related_to_self_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![Some(NullableOperation {
            operation_identifier: Some(NullableOperationIdentifier {
                index: 0,
                network_index: None,
            }),
            related_operations: vec![Some(NullableOperationIdentifier {
                index: 0,
                network_index: None,
            })],
            type_: "PAYMENT".into(),
            status: Some("SUCCESS".into()),
            account: valid_account.clone(),
            amount: valid_amount.clone(),
            ..Default::default()
        })],
        ..Default::default()
    });
    let out_of_order_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
        ],
        ..Default::default()
    });
    let related_to_later_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                related_operations: vec![Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
        ],
        ..Default::default()
    });
    let related_duplicate_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![
                    Some(NullableOperationIdentifier {
                        index: 0,
                        network_index: None,
                    }),
                    Some(NullableOperationIdentifier {
                        index: 0,
                        network_index: None,
                    }),
                ],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
        ],
        ..Default::default()
    });
    let related_missing_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 2,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
        ],
        ..Default::default()
    });
    let invalid_related_transaction = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
        ],
        related_transactions: vec![Some(NullableRelatedTransaction {
            network_identifier: Some(NetworkIdentifier {
                blockchain: "HELLO".into(),
                network: "WORLD".into(),
                sub_network_identifier: None,
            }),
            transaction_identifier: Some(TransactionIdentifier {
                hash: "blah".into(),
            }),
            direction: "blah".into(),
        })],
        ..Default::default()
    });
    let duplicated_related_transactions = Some(NullableTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(NullableOperation {
                operation_identifier: Some(NullableOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(NullableOperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            }),
        ],
        related_transactions: vec![
            Some(NullableRelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "HELLO".into(),
                    network: "WORLD".into(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: Some(TransactionIdentifier {
                    hash: "blah".into(),
                }),
                direction: "Forward".into(),
            }),
            Some(NullableRelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "HELLO".into(),
                    network: "WORLD".into(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: Some(TransactionIdentifier {
                    hash: "blah".into(),
                }),
                direction: "Forward".into(),
            }),
        ],
        ..Default::default()
    });

    let asserter = |extras: BlockTestExtras| {
        Asserter::new_client_with_responses(
            Some(NetworkIdentifier {
                blockchain: "HELLO".into(),
                network: "WORLD".into(),
                sub_network_identifier: None,
            }),
            Some(NullableNetworkStatusResponse {
                current_block_identifier: Some(NullableBlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: Some(NullableBlockIdentifier {
                    index: extras.genesis_index,
                    hash: format!("block {}", extras.genesis_index),
                }),
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Some(Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                })],
            }),
            Some(NullableNetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(NullableAllow {
                    operation_statuses: vec![
                        Some(OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        }),
                        Some(OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        }),
                    ],
                    operation_types: vec!["PAYMENT".into()],
                    timestamp_start_index: extras.start_index,
                    ..Default::default()
                }),
            }),
            extras.validation_file_path.as_ref(),
        )
        .unwrap()
    };

    let tests = vec![
        TestCase {
            name: "valid block",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![valid_transaction.clone()],
                    metadata: Default::default(),
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "valid block (before start index)",
            payload: MethodPayload {
                caller: asserter(BlockTestExtras {
                    start_index: Some(valid_block_ident.index + 1),
                    ..Default::default()
                }),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "genesis block (without start index)",
            payload: MethodPayload {
                caller: asserter(BlockTestExtras {
                    genesis_index: valid_block_ident.index,
                    ..Default::default()
                }),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "genesis block (with start index)",
            payload: MethodPayload {
                caller: asserter(BlockTestExtras {
                    genesis_index: genesis_ident.index,
                    start_index: Some(genesis_ident.index + 1),
                    ..Default::default()
                }),
                payload: Some(NullableBlock {
                    block_identifier: Some(genesis_ident.clone()),
                    parent_block_identifier: Some(genesis_ident.clone()),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "invalid genesis block (with start index)",
            payload: MethodPayload {
                caller: asserter(BlockTestExtras {
                    genesis_index: genesis_ident.index,
                    start_index: Some(genesis_ident.index),
                    ..Default::default()
                }),
                payload: Some(NullableBlock {
                    block_identifier: Some(genesis_ident.clone()),
                    parent_block_identifier: Some(genesis_ident),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::TimestampBeforeMin.into()),
        },
        TestCase {
            name: "out of order transaction operations",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![out_of_order_transaction],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        TestCase {
            name: "related to self transaction operations",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![related_to_self_transaction],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        TestCase {
            name: "related to later transaction operations",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![related_to_later_transaction],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        TestCase {
            name: "duplicate related transaction operations",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![related_duplicate_transaction],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::RelatedOperationIndexDuplicate.into()),
        },
        TestCase {
            name: "missing related transaction operations",
            payload: MethodPayload {
                caller: asserter(BlockTestExtras {
                    validation_file_path: Some(PathBuf::from(
                        "validation_balanced_related_ops.json",
                    )),
                    ..Default::default()
                }),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![related_missing_transaction],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::RelatedOperationMissing.into()),
        },
        TestCase {
            name: "nil block",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: None,
            },
            criteria: Some(BlockError::BlockIsNil.into()),
        },
        TestCase {
            name: "nil block hash",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: None,
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        TestCase {
            name: "invalid block hash",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(Default::default()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        TestCase {
            name: "block previous hash missing",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(Default::default()),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        TestCase {
            name: "invalid parent block index",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(NullableBlockIdentifier {
                        index: valid_block_ident.index,
                        hash: valid_parent_block_ident.hash.clone(),
                    }),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::BlockIndexPrecedesParentBlockIndex.into()),
        },
        TestCase {
            name: "invalid parent block hash",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(NullableBlockIdentifier {
                        index: valid_parent_block_ident.index,
                        hash: valid_block_ident.hash.clone(),
                    }),
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::BlockHashEqualsParentBlockHash.into()),
        },
        TestCase {
            name: "invalid block timestamp less than MinUnixEpoch",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::TimestampBeforeMin.into()),
        },
        TestCase {
            name: "invalid block timestamp greater than MaxUnixEpoch",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    transactions: vec![valid_transaction],
                    timestamp: (MAX_UNIX_EPOCH + 1),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::TimestampAfterMax.into()),
        },
        TestCase {
            name: "invalid block transaction",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    transactions: vec![Some(Default::default())],
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::TxIdentifierIsNil.into()),
        },
        TestCase {
            name: "invalid related transaction",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident.clone()),
                    parent_block_identifier: Some(valid_parent_block_ident.clone()),
                    transactions: vec![invalid_related_transaction],
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::InvalidDirection.into()),
        },
        TestCase {
            name: "duplicate related transaction",
            payload: MethodPayload {
                caller: asserter(Default::default()),
                payload: Some(NullableBlock {
                    block_identifier: Some(valid_block_ident),
                    parent_block_identifier: Some(valid_parent_block_ident),
                    transactions: vec![duplicated_related_transactions],
                    timestamp: (MIN_UNIX_EPOCH + 1),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::DuplicateRelatedTransaction.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| t.caller.block(t.payload.as_ref()));
}
