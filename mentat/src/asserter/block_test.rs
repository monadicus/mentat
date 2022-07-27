use std::path::{Path, PathBuf};

use super::*;

#[test]
fn test_block_identifier() {
    let tests = [
        AsserterTest {
            name: "valid identifier",
            payload: Some(BlockIdentifier {
                index: 1,
                hash: "block 1".into(),
            }),
            err: None,
        },
        AsserterTest {
            name: "nil identifier",
            payload: None,
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "invalid index",
            payload: Some(BlockIdentifier {
                index: -1,
                hash: "block 1".into(),
            }),
            err: Some(BlockError::BlockIdentifierIndexIsNeg.into()),
        },
        AsserterTest {
            name: "invalid hash",
            payload: Some(BlockIdentifier {
                index: 1,
                hash: String::new(),
            }),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    ];

    AsserterTest::run(&tests, block_identifier);
}

#[test]
fn test_amount() {
    let tests = [
        AsserterTest {
            name: "valid amount",
            payload: Some(Amount {
                value: "100000".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "valid amount no decimals",
            payload: Some(Amount {
                value: "100000".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: Default::default(),
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "valid negative amount",
            payload: Some(Amount {
                value: "-100000".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "nil amount",
            payload: None,
            err: Some(BlockError::AmountValueMissing.into()),
        },
        AsserterTest {
            name: "nil currency",
            payload: Some(Amount {
                value: "-100000".into(),
                currency: None,
                metadata: Default::default(),
            }),
            err: Some(BlockError::AmountCurrencyIsNil.into()),
        },
        AsserterTest {
            name: "invalid non number",
            payload: Some(Amount {
                value: "blah".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "{}: blah",
                BlockError::AmountIsNotInt
            ))),
        },
        AsserterTest {
            name: "invalid integer format",
            payload: Some(Amount {
                value: "1.0".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "{}: 1.0",
                BlockError::AmountIsNotInt
            ))),
        },
        AsserterTest {
            name: "invalid non-integer",
            payload: Some(Amount {
                value: "1.1".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: Some(AsserterError::from(format!(
                "{}: 1.1",
                BlockError::AmountIsNotInt
            ))),
        },
        AsserterTest {
            name: "invalid symbol",
            payload: Some(Amount {
                value: "11".into(),
                currency: Some(Currency {
                    symbol: String::new(),
                    decimals: 1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: Some(BlockError::AmountCurrencySymbolEmpty.into()),
        },
        AsserterTest {
            name: "invalid decimals",
            payload: Some(Amount {
                value: "111".into(),
                currency: Some(Currency {
                    symbol: "BTC".into(),
                    decimals: -1,
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: Some(BlockError::AmountCurrencyHasNegDecimals.into()),
        },
    ];

    AsserterTest::run(&tests, amount);
}

#[derive(Default)]
struct OperationIdentTest {
    ident: Option<OperationIdentifier>,
    index: i64,
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

    let tests = [
        AsserterTest {
            name: "valid identifier",
            payload: Some(OperationIdentTest {
                ident: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                index: 0,
            }),
            err: None,
        },
        AsserterTest {
            name: "nil identifier",
            payload: Some(OperationIdentTest {
                ident: None,
                index: 0,
            }),
            err: Some(BlockError::OperationIdentifierIndexIsNil.into()),
        },
        AsserterTest {
            name: "out-of-order index",
            payload: Some(OperationIdentTest {
                ident: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                index: 1,
            }),
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "valid identifier with network index",
            payload: Some(OperationIdentTest {
                ident: Some(OperationIdentifier {
                    index: 0,
                    network_index: Some(valid_network_index),
                }),
                index: 0,
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid identifier with network index",
            payload: Some(OperationIdentTest {
                ident: Some(OperationIdentifier {
                    index: 0,
                    network_index: Some(invalid_network_index),
                }),
                index: 0,
            }),
            err: Some(BlockError::OperationIdentifierNetworkIndexInvalid.into()),
        },
    ];

    AsserterTest::run(&tests, |t| t.unwrap().run());
}

#[test]
fn test_account_identifier() {
    let tests = [
        AsserterTest {
            name: "valid identifier",
            payload: Some(AccountIdentifier {
                address: "acct1".into(),
                sub_account: None,
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid identifier",
            payload: Some(AccountIdentifier {
                address: Default::default(),
                sub_account: None,
                metadata: Default::default(),
            }),
            err: Some(BlockError::AccountAddrMissing.into()),
        },
        AsserterTest {
            name: "valid identifier with subaccount",
            payload: Some(AccountIdentifier {
                address: "acct1".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "acct2".into(),
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid identifier with subaccount",
            payload: Some(AccountIdentifier {
                address: "acct1".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: String::new(),
                    metadata: Default::default(),
                }),
                metadata: Default::default(),
            }),
            err: Some(BlockError::AccountSubAccountAddrMissing.into()),
        },
    ];

    AsserterTest::run(&tests, account_identifier);
}

#[derive(Default)]
struct OperationValidationsTest {
    operations: Vec<Option<Operation>>,
    construction: bool,
}

#[test]
fn test_operations_validations() {
    let valid_deposit_amt = Some(Amount {
        value: "1000".into(),
        currency: Some(Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_withdraw_amt = Some(Amount {
        value: "-1000".into(),
        currency: Some(Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_fee_amt = Some(Amount {
        value: "-100".into(),
        currency: Some(Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let invalid_fee_amt = Some(Amount {
        value: "100".into(),
        currency: Some(Currency {
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

    let tests = [
        CustomAsserterTest {
            name: "valid operations based on validation file",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_withdraw_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_balanced.json")),
            err: None,
        },
        CustomAsserterTest {
            name: "throw error on missing fee operation",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_balanced.json")),
            err: Some(BlockError::FeeCountMismatch.into()),
        },
        CustomAsserterTest {
            name: "throw error on missing payment operation",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_balanced.json")),
            err: Some(BlockError::PaymentCountMismatch.into()),
        },
        CustomAsserterTest {
            name: "throw error on payment amount not balancing",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Some(Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            }),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_balanced.json")),
            err: Some(BlockError::PaymentAmountNotBalancing.into()),
        },
        CustomAsserterTest {
            name: "valid operations based on validation file - unbalanced",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Some(Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            }),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_unbalanced.json")),
            err: None,
        },
        CustomAsserterTest {
            name: "fee operation shouldn't contain related_operation key",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Some(Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            }),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 2,
                            network_index: None,
                        }),
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_fee_amt.clone(),
                        related_operations: vec![Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        })],
                        ..Default::default()
                    }),
                ],
                construction: false,
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_unbalanced.json")),
            err: Some(BlockError::RelatedOperationInFeeNotAllowed.into()),
        },
        CustomAsserterTest {
            name: "fee amount is non-negative",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt.clone(),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Some(Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            }),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_unbalanced.json")),
            err: Some(BlockError::FeeAmountNotNegative.into()),
        },
        CustomAsserterTest {
            name: "fee amount is negative as expected",
            payload: Some(OperationValidationsTest {
                operations: vec![
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: valid_deposit_amt,
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            network_index: None,
                        }),
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account.clone(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Some(Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            }),
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: Some(PathBuf::from("validation_fee_and_payment_unbalanced.json")),
            err: None,
        },
    ];

    let asserter = |validation_file_path: &Option<PathBuf>| {
        Asserter::new_client_with_responses(
            Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            }),
            Some(NetworkStatusResponse {
                current_block_identifier: Some(BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: Some(BlockIdentifier {
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
            Some(NetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(Allow {
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
            validation_file_path.as_deref(),
        )
        .unwrap()
    };

    CustomAsserterTest::custom_asserter_tests(&tests, asserter, |asserter, payload| {
        let payload = payload.unwrap();
        asserter.operations(&payload.operations, payload.construction)
    });
}

#[derive(Default)]
struct OperationTest {
    operation: Option<Operation>,
    index: i64,
    successful: bool,
    construction: bool,
}

#[test]
fn test_operation() {
    let valid_amount = Some(Amount {
        value: "1000".into(),
        currency: Some(Currency {
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

    let tests = [
        CustomAsserterTest {
            name: "valid operation",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "valid operation no account",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "nil operation",
            payload: Some(OperationTest {
                operation: None,
                index: 1,
                successful: false,
                construction: false,
            }),
            extras: (),
            err: Some(BlockError::OperationIsNil.into()),
        },
        CustomAsserterTest {
            name: "invalid operation no account",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: Some(BlockError::AccountIsNil.into()),
        },
        CustomAsserterTest {
            name: "invalid operation empty account",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: Some(BlockError::AccountAddrMissing.into()),
        },
        CustomAsserterTest {
            name: "invalid operation invalid index",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        CustomAsserterTest {
            name: "invalid operation invalid type",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        CustomAsserterTest {
            name: "unsuccessful operation",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "invalid operation invalid status",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: Some(BlockError::OperationStatusInvalid.into()),
        },
        CustomAsserterTest {
            name: "valid construction operation",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "valid construction operation (empty status)",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "invalid construction operation",
            payload: Some(OperationTest {
                operation: Some(Operation {
                    operation_identifier: Some(OperationIdentifier {
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
            }),
            extras: (),
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
    ];

    let asserter = |_: &()| {
        Asserter::new_client_with_responses(
            Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            }),
            Some(NetworkStatusResponse {
                current_block_identifier: Some(BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: Some(BlockIdentifier {
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
            Some(NetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(Allow {
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
        .unwrap()
    };

    let test_count = tests.len();
    let failed = tests
        .into_iter()
        .map(|test| {
            println!("{}: ", test.name);

            print!("Testing operation: ");
            let asserter = asserter(&test.extras);
            let payload = test.payload.as_ref().unwrap();
            if !assert_correct(
                &test.err,
                &asserter.operation(
                    payload.operation.as_ref(),
                    payload.index,
                    payload.construction,
                ),
            ) {
                return false;
            }

            if test.err.is_none() && !payload.construction {
                print!("Testing operation successful: ");
                let successful = asserter
                    .operation_successful(payload.operation.as_ref())
                    .unwrap();

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
    genesis_index: i64,
    start_index: Option<i64>,
    validation_file_path: Option<PathBuf>,
}

#[test]
fn test_block() {
    let genesis_ident = BlockIdentifier {
        hash: "gen".into(),
        index: 0,
    };
    let valid_block_ident = BlockIdentifier {
        hash: "blah".into(),
        index: 100,
    };
    let valid_parent_block_ident = BlockIdentifier {
        hash: "blah parent".into(),
        index: 99,
    };
    let valid_amount = Some(Amount {
        value: "1000".into(),
        currency: Some(Currency {
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
    let valid_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(OperationIdentifier {
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
        related_transactions: vec![Some(RelatedTransaction {
            network_identifier: Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            }),
            transaction_identifier: Some(TransactionIdentifier {
                hash: "blah".into(),
            }),
            direction: Direction::FORWARD.into(),
        })],
        metadata: Default::default(),
    });
    let related_to_self_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![Some(Operation {
            operation_identifier: Some(OperationIdentifier {
                index: 0,
                network_index: None,
            }),
            related_operations: vec![Some(OperationIdentifier {
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
    let out_of_order_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
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
    let related_to_later_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                related_operations: vec![Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(OperationIdentifier {
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
    let related_duplicate_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![
                    Some(OperationIdentifier {
                        index: 0,
                        network_index: None,
                    }),
                    Some(OperationIdentifier {
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
    let related_missing_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
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
    let invalid_related_transaction = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(OperationIdentifier {
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
        related_transactions: vec![Some(RelatedTransaction {
            network_identifier: Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            }),
            transaction_identifier: Some(TransactionIdentifier {
                hash: "blah".into(),
            }),
            direction: "blah".into(),
        })],
        ..Default::default()
    });
    let duplicated_related_transactions = Some(Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            }),
            Some(Operation {
                operation_identifier: Some(OperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(OperationIdentifier {
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
            Some(RelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "hello".into(),
                    network: "world".into(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: Some(TransactionIdentifier {
                    hash: "blah".into(),
                }),
                direction: "Forward".into(),
            }),
            Some(RelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "hello".into(),
                    network: "world".into(),
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

    let tests = [
        CustomAsserterTest {
            name: "valid block",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![valid_transaction.clone()],
                metadata: Default::default(),
            }),
            extras: Default::default(),
            err: None,
        },
        CustomAsserterTest {
            name: "valid block (before start index)",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: BlockTestExtras {
                start_index: Some(valid_block_ident.index + 1),
                ..Default::default()
            },
            err: None,
        },
        CustomAsserterTest {
            name: "genesis block (without start index)",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: BlockTestExtras {
                genesis_index: valid_block_ident.index,
                ..Default::default()
            },
            err: None,
        },
        CustomAsserterTest {
            name: "genesis block (with start index)",
            payload: Some(Block {
                block_identifier: Some(genesis_ident.clone()),
                parent_block_identifier: Some(genesis_ident.clone()),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: BlockTestExtras {
                genesis_index: genesis_ident.index,
                start_index: Some(genesis_ident.index + 1),
                ..Default::default()
            },
            err: None,
        },
        CustomAsserterTest {
            name: "invalid genesis block (with start index)",
            payload: Some(Block {
                block_identifier: Some(genesis_ident.clone()),
                parent_block_identifier: Some(genesis_ident.clone()),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: BlockTestExtras {
                genesis_index: genesis_ident.index,
                start_index: Some(genesis_ident.index),
                ..Default::default()
            },
            err: Some(BlockError::TimestampBeforeMin.into()),
        },
        CustomAsserterTest {
            name: "out of order transaction operations",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![out_of_order_transaction],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        CustomAsserterTest {
            name: "related to self transaction operations",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![related_to_self_transaction],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        CustomAsserterTest {
            name: "related to later transaction operations",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![related_to_later_transaction],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        CustomAsserterTest {
            name: "duplicate related transaction operations",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![related_duplicate_transaction],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::RelatedOperationIndexDuplicate.into()),
        },
        CustomAsserterTest {
            name: "missing related transaction operations",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![related_missing_transaction],
                ..Default::default()
            }),
            extras: BlockTestExtras {
                validation_file_path: Some(PathBuf::from("validation_balanced_related_ops.json")),
                ..Default::default()
            },
            err: Some(BlockError::RelatedOperationMissing.into()),
        },
        CustomAsserterTest {
            name: "nil block",
            payload: Default::default(),
            extras: Default::default(),
            err: Some(BlockError::BlockIsNil.into()),
        },
        CustomAsserterTest {
            name: "nil block hash",
            payload: Some(Block {
                block_identifier: None,
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        CustomAsserterTest {
            name: "invalid block hash",
            payload: Some(Block {
                block_identifier: Some(Default::default()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        CustomAsserterTest {
            name: "block previous hash missing",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(Default::default()),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        CustomAsserterTest {
            name: "invalid parent block index",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(BlockIdentifier {
                    index: valid_block_ident.index,
                    hash: valid_parent_block_ident.hash.clone(),
                }),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::BlockIndexPrecedesParentBlockIndex.into()),
        },
        CustomAsserterTest {
            name: "invalid parent block hash",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(BlockIdentifier {
                    index: valid_parent_block_ident.index,
                    hash: valid_block_ident.hash.clone(),
                }),
                timestamp: (MIN_UNIX_EPOCH + 1),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::BlockHashEqualsParentBlockHash.into()),
        },
        CustomAsserterTest {
            name: "invalid block timestamp less than MinUnixEpoch",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::TimestampBeforeMin.into()),
        },
        CustomAsserterTest {
            name: "invalid block timestamp greater than MaxUnixEpoch",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                transactions: vec![valid_transaction],
                timestamp: (MAX_UNIX_EPOCH + 1),
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::TimestampAfterMax.into()),
        },
        CustomAsserterTest {
            name: "invalid block transaction",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                transactions: vec![Some(Default::default())],
                timestamp: (MIN_UNIX_EPOCH + 1),
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::TxIdentifierIsNil.into()),
        },
        CustomAsserterTest {
            name: "invalid related transaction",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident.clone()),
                parent_block_identifier: Some(valid_parent_block_ident.clone()),
                transactions: vec![invalid_related_transaction],
                timestamp: (MIN_UNIX_EPOCH + 1),
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::InvalidDirection.into()),
        },
        CustomAsserterTest {
            name: "duplicate related transaction",
            payload: Some(Block {
                block_identifier: Some(valid_block_ident),
                parent_block_identifier: Some(valid_parent_block_ident),
                transactions: vec![duplicated_related_transactions],
                timestamp: (MIN_UNIX_EPOCH + 1),
                ..Default::default()
            }),
            extras: Default::default(),
            err: Some(BlockError::DuplicateRelatedTransaction.into()),
        },
    ];

    let asserter = |extras: &BlockTestExtras| {
        Asserter::new_client_with_responses(
            Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            }),
            Some(NetworkStatusResponse {
                current_block_identifier: Some(BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: Some(BlockIdentifier {
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
            Some(NetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(Allow {
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
            extras.validation_file_path.as_deref(),
        )
        .unwrap()
    };

    CustomAsserterTest::custom_asserter_tests(&tests, asserter, Asserter::block);
}
