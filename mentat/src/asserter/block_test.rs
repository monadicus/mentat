use std::path::PathBuf;

use super::{
    server_test::valid_account_identifier,
    test_utils::{non_asserter_tests, AsserterTest},
};
use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::{
            account_identifier, amount, block_identifier, operation_identifier, MAX_UNIX_EPOCH,
            MIN_UNIX_EPOCH,
        },
        errors::{AsserterError, BlockError},
        tests::test_utils::AsserterTester,
    },
    types::{
        AccountIdentifier, Allow, Amount, Block, BlockIdentifier, Currency, Direction,
        NetworkIdentifier, NetworkOptionsResponse, NetworkStatusResponse, Operation,
        OperationIdentifier, OperationStatus, Peer, RelatedTransaction, SubAccountIdentifier,
        Transaction, TransactionIdentifier, Version,
    },
};

#[test]
fn test_block_identifier() {
    let tests = [
        AsserterTest {
            name: "valid identifier",
            payload: BlockIdentifier {
                index: 1,
                hash: "block 1".into(),
            },
            err: None,
        },
        // TODO allow None BlockIdent
        // "nil identifier" => ServerTest {
        //     request: None,
        //     err: Some(BlockError::BlockIdentifierIsNil.into())
        // },
        // TODO make block ident index i64
        // "invalid index" => ServerTest {
        //     request: BlockIdentifier {
        // 		index: -1,
        // 		hash: "block 1".into()
        // 	},
        //     err: Some(BlockError::BlockIdentifierIndexIsNeg.into())
        // },
        AsserterTest {
            name: "invalid hash",
            payload: BlockIdentifier {
                index: 1,
                hash: String::new(),
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    ];

    non_asserter_tests(&tests, block_identifier);
}

#[test]
fn test_amount() {
    let tests = [
        AsserterTest {
            name: "valid amount",
            payload: Some(Amount {
                value: "100000".into(),
                currency: Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "valid amount no decimals",
            payload: Some(Amount {
                value: "100000".into(),
                currency: Currency {
                    symbol: "BTC".into(),
                    decimals: Default::default(),
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "valid negative amount",
            payload: Some(Amount {
                value: "-100000".into(),
                currency: Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "nil amount",
            payload: None,
            err: Some(BlockError::AmountValueMissing.into()),
        },
        // Allow None currency
        // "nil currency" => ServerTest {
        // 	request: Some(Amount {
        // 		value: "-100000".into(),
        // 		currency: None,
        // 		metadata: Default::default(),
        // 	}),
        // 	err: Some(BlockError::AmountCurrencyIsNil.into())
        // },
        AsserterTest {
            name: "invalid non number",
            payload: Some(Amount {
                value: "blah".into(),
                currency: Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                },
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
                currency: Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                },
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
                currency: Currency {
                    symbol: "BTC".into(),
                    decimals: 1,
                    metadata: Default::default(),
                },
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
                currency: Currency {
                    symbol: String::new(),
                    decimals: 1,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: Some(BlockError::AmountCurrencySymbolEmpty.into()),
        },
        // TODO make decimals i64
        // "invalid decimals" => ServerTest {
        // 	request: Amount {
        // 		value: "1.0".into(),
        // 		currency: Currency {
        // 			symbol: "BTC".into(),
        // 			decimals: -1,
        // 			metadata: Default::default(),
        // 		},
        // 		metadata: Default::default(),
        // 	},
        // 	err: Some(BlockError::AmountCurrencyHasNegDecimals.into())
        // },
    ];

    non_asserter_tests(&tests, |data| amount(data.as_ref()));
}

#[derive(Default)]
struct OperationIdentTest {
    ident: OperationIdentifier,
    index: i64,
}

#[test]
fn test_operation_identifier() {
    let valid_network_index = 1;
    // TODO make network indexs i64
    // let invalid_network_index = -1;

    let tests = [
        AsserterTest {
            name: "valid identifier",
            payload: OperationIdentTest {
                ident: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                index: 0,
            },
            err: None,
        },
        // TODO allow None OperationIdentifier
        // "nil identifier" => OperationIdentTest {
        // 	request: None,
        // 	index: 0,
        // 	err: Some(BlockError::OperationIdentifierIndexIsNil.into()),
        // },
        AsserterTest {
            name: "out-of-order index",
            payload: OperationIdentTest {
                ident: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                index: 1,
            },
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "valid identifier with network index",
            payload: OperationIdentTest {
                ident: OperationIdentifier {
                    index: 0,
                    network_index: Some(valid_network_index),
                },
                index: 0,
            },
            err: None,
        },
        // TODO see invalid_network_index defined above
        // "invalid identifier with network index" => OperationIdentTest {
        // 	request: OperationIdentTest {
        // ident: OperationIdentifier {
        // 		index: 0,
        // 		network_index: Some(invalid_network_index),
        // 	},
        // 	index: 0,
        // },
        // 	err: Some(BlockError::OperationIdentifierNetworkIndexInvalid.into())
        // },
    ];

    non_asserter_tests(&tests, |data| operation_identifier(&data.ident, data.index));
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

    non_asserter_tests(&tests, |data| account_identifier(data.as_ref()));
}

#[derive(Default)]
struct OperationValidationsTest {
    operations: Vec<Operation>,
    validation_file_path: PathBuf,
    construction: bool,
}

#[test]
fn test_operation_validations() {
    let valid_deposit_amt = Amount {
        value: "1000".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let valid_withdraw_amt = Amount {
        value: "-1000".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let valid_fee_amt = Amount {
        value: "-100".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let invalid_fee_amt = Amount {
        value: "100".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };

    let tests = [
        AsserterTest {
            name: "valid operations based on validation file",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_withdraw_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_fee_amt.clone()),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_balanced.json",
                ),
                construction: false,
            },
            err: None,
        },
        AsserterTest {
            name: "throw error on missing fee operation",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_withdraw_amt),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_balanced.json",
                ),
                construction: false,
            },
            err: Some(BlockError::FeeCountMismatch.into()),
        },
        AsserterTest {
            name: "throw error on missing payment operation",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_fee_amt.clone()),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_balanced.json",
                ),
                construction: false,
            },
            err: Some(BlockError::PaymentCountMismatch.into()),
        },
        AsserterTest {
            name: "throw error on payment amount not balancing",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            },
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_fee_amt.clone()),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_balanced.json",
                ),
                construction: false,
            },
            err: Some(BlockError::PaymentAmountNotBalancing.into()),
        },
        AsserterTest {
            name: "valid operations based on validation file - unbalanced",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            },
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_fee_amt.clone()),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_unbalanced.json",
                ),
                construction: false,
            },
            err: None,
        },
        AsserterTest {
            name: "fee operation shouldn't contain related_operation key",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            },
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_fee_amt.clone()),
                        related_operations: Some(vec![OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }]),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_unbalanced.json",
                ),
                construction: false,
            },
            err: Some(BlockError::RelatedOperationInFeeNotAllowed.into()),
        },
        AsserterTest {
            name: "fee amount is non-negative",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt.clone()),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            },
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(invalid_fee_amt),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_unbalanced.json",
                ),
                construction: false,
            },
            err: Some(BlockError::FeeAmountNotNegative.into()),
        },
        AsserterTest {
            name: "fee amount is negative as expected",
            payload: OperationValidationsTest {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_deposit_amt),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "PAYMENT".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(Amount {
                            value: "-2000".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                metadata: Default::default(),
                            },
                            metadata: Default::default(),
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "FEE".into(),
                        status: Some("SUCCESS".into()),
                        account: valid_account_identifier(),
                        amount: Some(valid_fee_amt),
                        related_operations: Some(vec![OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }]),
                        ..Default::default()
                    },
                ],
                validation_file_path: PathBuf::from(
                    "data/validation_fee_and_payment_unbalanced.json",
                ),
                construction: false,
            },
            err: None,
        },
    ];

    // TODO Asserter tester
    tests.into_iter().for_each(|test| {
        test.print();

        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: 0,
                    hash: "block 0".into(),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".into(), "FEE".into()],
                    ..Default::default()
                },
            },
            test.payload.validation_file_path,
        )
        .unwrap();

        todo!()
        // let resp = asserter.unwrap().operations(&test.operations,
        // test.construction);
    });
}

#[derive(Default)]
struct OperationTest {
    operation: Operation,
    index: i64,
    successful: bool,
    construction: bool,
}

#[test]
fn test_operation() {
    let valid_amount = Some(Amount {
        value: "1000".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    });

    let tests = [
        AsserterTest {
            name: "valid operation",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    account: valid_account_identifier(),
                    amount: valid_amount.clone(),
                    ..Default::default()
                },
                index: 1,
                successful: true,
                construction: false,
            },
            err: None,
        },
        AsserterTest {
            name: "valid operation no account",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    amount: valid_amount.clone(),
                    ..Default::default()
                },
                index: 1,
                successful: true,
                construction: false,
            },
            err: None,
        },
        // TODO allow nill operations
        // "nil operation" => ServerTest {
        //     OperationTest {
        //         operation: None,
        //         index: 1,
        //         successful: false,
        //         construction: false,
        //     },
        //     err: Some(BlockError::OperationIsNil.into()),
        // },
        AsserterTest {
            name: "invalid operation no account",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    amount: valid_amount.clone(),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: false,
            },
            err: Some(BlockError::OperationIsNil.into()),
        },
        AsserterTest {
            name: "invalid operation empty account",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    account: Some(AccountIdentifier::default()),
                    amount: valid_amount.clone(),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: false,
            },
            err: Some(BlockError::AccountAddrMissing.into()),
        },
        AsserterTest {
            name: "invalid operation invalid index",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    ..Default::default()
                },
                index: 2,
                successful: false,
                construction: false,
            },
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "invalid operation invalid type",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "STAKE".into(),
                    status: Some("SUCCESS".into()),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: false,
            },
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterTest {
            name: "unsuccessful operation",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("FAILURE".into()),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: false,
            },
            err: None,
        },
        AsserterTest {
            name: "invalid operation invalid status",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("DEFERRED".into()),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: false,
            },
            err: Some(BlockError::OperationStatusInvalid.into()),
        },
        AsserterTest {
            name: "valid construction operation",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    account: valid_account_identifier(),
                    amount: valid_amount.clone(),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: true,
            },
            err: None,
        },
        AsserterTest {
            name: "valid construction operation (empty status)",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some(String::new()),
                    account: valid_account_identifier(),
                    amount: valid_amount.clone(),
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: true,
            },
            err: None,
        },
        AsserterTest {
            name: "invalid construction operation",
            payload: OperationTest {
                operation: Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    type_: "PAYMENT".into(),
                    status: Some("SUCCESS".into()),
                    account: valid_account_identifier(),
                    amount: valid_amount,
                    ..Default::default()
                },
                index: 1,
                successful: false,
                construction: true,
            },
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
    ];

    tests.into_iter().for_each(|test| {
        test.print();

        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: 0,
                    hash: "block 0".into(),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".into()],
                    ..Default::default()
                },
            },
            "".into(),
        )
        .unwrap();
        todo!()
        // let resp = asserter.unwrap().operation(&test.operation, test.index,
        // test.construction);
    });
}

#[derive(Default)]
struct BlockTest {
    block: Block,
    // TODO consider making this an options
    validation_file_path: PathBuf,
    genesis_index: i64,
    start_index: Option<i64>,
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
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    });
    let valid_account = Some(AccountIdentifier {
        address: "test".into(),
        ..Default::default()
    });
    let valid_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(vec![OperationIdentifier {
                    index: 0,
                    network_index: None,
                }]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        related_transactions: Some(vec![RelatedTransaction {
            network_identifier: Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            }),
            transaction_identifier: TransactionIdentifier {
                hash: "blah".into(),
            },
            direction: Direction::Forward,
        }]),
        metadata: Default::default(),
    };
    let related_to_self_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![Operation {
            operation_identifier: OperationIdentifier {
                index: 0,
                network_index: None,
            },
            related_operations: Some(vec![OperationIdentifier {
                index: 0,
                network_index: None,
            }]),
            type_: "PAYMENT".into(),
            status: Some("SUCCESS".into()),
            account: valid_account_identifier(),
            amount: valid_amount.clone(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let out_of_order_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(vec![OperationIdentifier {
                    index: 0,
                    network_index: None,
                }]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let related_to_later_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                related_operations: Some(vec![OperationIdentifier {
                    index: 1,
                    network_index: None,
                }]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(vec![OperationIdentifier {
                    index: 0,
                    network_index: None,
                }]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let related_duplicate_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(vec![
                    OperationIdentifier {
                        index: 0,
                        network_index: None,
                    },
                    OperationIdentifier {
                        index: 0,
                        network_index: None,
                    },
                ]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let related_missing_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(Vec::new()),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(Vec::new()),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    // TODO allow arbitrary directions
    // let invalid_related_transaction = Transaction {
    //     transaction_identifier: TransactionIdentifier {
    //         hash: "blah".into(),
    //     },
    //     operations: vec![
    //         Operation {
    //             operation_identifier: OperationIdentifier {
    //                 index: 0,
    //                 network_index: None,
    //             },
    //             type_: "PAYMENT".into(),
    //             status: Some("SUCCESS".into()),
    //             account: valid_account_identifier(),
    //             amount: valid_amount.clone(),
    //             ..Default::default()
    //         },
    //         Operation {
    //             operation_identifier: OperationIdentifier {
    //                 index: 1,
    //                 network_index: None,
    //             },
    //             related_operations: Some(vec![OperationIdentifier {
    //                 index: 0,
    //                 network_index: None,
    //             }]),
    //             type_: "PAYMENT".into(),
    //             status: Some("SUCCESS".into()),
    //             account: valid_account_identifier(),
    //             amount: valid_amount.clone(),
    //             ..Default::default()
    //         },
    //     ],
    //     related_transactions: Some(vec![RelatedTransaction {
    //         network_identifier: Some(NetworkIdentifier {
    //             blockchain: "hello".into(),
    //             network: "world".into(),
    //             sub_network_identifier: None,
    //         }),
    //         transaction_identifier: TransactionIdentifier {
    //             hash: "blah".into(),
    //         },
    //         direction: "blah",
    //     }]),
    //     ..Default::default()
    // };
    let duplicated_related_transactions = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".into(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account_identifier(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(vec![OperationIdentifier {
                    index: 0,
                    network_index: None,
                }]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            },
        ],
        related_transactions: Some(vec![
            RelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "hello".into(),
                    network: "world".into(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: TransactionIdentifier {
                    hash: "blah".into(),
                },
                direction: Direction::Forward,
            },
            RelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "hello".into(),
                    network: "world".into(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: TransactionIdentifier {
                    hash: "blah".into(),
                },
                direction: Direction::Forward,
            },
        ]),
        ..Default::default()
    };

    // TODO shouldn't need any cast operations below
    let tests = [
        AsserterTest {
            name: "valid block",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![valid_transaction.clone()],
                    metadata: Default::default(),
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: None,
        },
        AsserterTest {
            name: "valid block (before start index)",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: Some((valid_block_ident.index + 1) as i64),
            },
            err: None,
        },
        AsserterTest {
            name: "genesis block (without start index)",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: valid_block_ident.index as i64,
                start_index: None,
            },
            err: None,
        },
        AsserterTest {
            name: "genesis block (with start index)",
            payload: BlockTest {
                block: Block {
                    block_identifier: genesis_ident.clone(),
                    parent_block_identifier: genesis_ident.clone(),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: genesis_ident.index as i64,
                start_index: Some((genesis_ident.index + 1) as i64),
            },
            err: None,
        },
        AsserterTest {
            name: "invalid genesis block (with start index)",
            payload: BlockTest {
                block: Block {
                    block_identifier: genesis_ident.clone(),
                    parent_block_identifier: genesis_ident.clone(),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: genesis_ident.index as i64,
                start_index: Some(genesis_ident.index as i64),
            },
            err: Some(BlockError::TimestampBeforeMin.into()),
        },
        AsserterTest {
            name: "out of order transaction operations",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![out_of_order_transaction],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "related to self transaction operations",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![related_to_self_transaction],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "related to later transaction operations",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![related_to_later_transaction],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "duplicate related transaction operations",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![related_duplicate_transaction],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::RelatedOperationIndexDuplicate.into()),
        },
        AsserterTest {
            name: "missing related transaction operations",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![related_missing_transaction],
                    ..Default::default()
                },
                validation_file_path: PathBuf::from("data/validation_balanced_related_ops.json"),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::RelatedOperationMissing.into()),
        },
        // TODO allow None block
        // "nil block" => ServerTest {
        // request: BlockTest {
        //     block: None,
        //     validation_file_path:  Default::default(),
        //     genesis_index: 0,
        //     start_index: None,
        // },
        //     err: Some(BlockError::BlockIsNil.into()),
        // },
        // TODO allow block ident to be None
        // "nil block hash" => ServerTest {
        // request: BlockTest {
        //     block: Block {
        //         block_identifier: None,
        //         parent_block_identifier: valid_parent_block_ident.clone(),
        //         timestamp: (MIN_UNIX_EPOCH + 1) as u64,
        //         transactions: vec![valid_transaction.clone()],
        //         ..Default::default()
        //     },
        //     validation_file_path:  Default::default(),
        //     genesis_index: 0,
        //     start_index: None,
        // },
        //     err: Some(BlockError::BlockIdentifierIsNil.into()),
        // },
        AsserterTest {
            name: "invalid block hash",
            payload: BlockTest {
                block: Block {
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        AsserterTest {
            name: "block previous hash missing",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        AsserterTest {
            name: "invalid parent block index",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: BlockIdentifier {
                        index: valid_block_ident.index,
                        hash: valid_parent_block_ident.hash.clone(),
                    },
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::BlockIndexPrecedesParentBlockIndex.into()),
        },
        AsserterTest {
            name: "invalid parent block hash",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: BlockIdentifier {
                        index: valid_parent_block_ident.index,
                        hash: valid_block_ident.hash.clone(),
                    },
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::BlockHashEqualsParentBlockHash.into()),
        },
        AsserterTest {
            name: "invalid block timestamp less than MinUnixEpoch",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    transactions: vec![valid_transaction.clone()],
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::TimestampBeforeMin.into()),
        },
        AsserterTest {
            name: "invalid block timestamp greater than MaxUnixEpoch",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    transactions: vec![valid_transaction],
                    timestamp: (MAX_UNIX_EPOCH + 1) as u64,
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::TimestampAfterMax.into()),
        },
        AsserterTest {
            name: "invalid block transaction",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident.clone(),
                    parent_block_identifier: valid_parent_block_ident.clone(),
                    transactions: vec![Default::default()],
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::TxIdentifierIsNil.into()),
        },
        // TODO see invalid_related_transaction defined above
        // "invalid related transaction" => ServerTest {
        //     request: BlockTest {
        //     block: Block {
        //         block_identifier: valid_block_ident.clone(),
        //         parent_block_identifier: valid_parent_block_ident.clone(),
        //         transactions: vec![invalid_related_transaction],
        //         timestamp: (MIN_UNIX_EPOCH + 1) as u64,
        //         ..Default::default()
        //     },
        //     validation_file_path: Default::default(),
        //     genesis_index: 0,
        //     start_index: None,
        // },
        //     err: Some(BlockError::InvalidDirection.into()),
        // },
        // TODO see invalid_related_transaction defined above
        AsserterTest {
            name: "invalid related transaction",
            payload: BlockTest {
                block: Block {
                    block_identifier: valid_block_ident,
                    parent_block_identifier: valid_parent_block_ident,
                    transactions: vec![duplicated_related_transactions],
                    timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                    ..Default::default()
                },
                validation_file_path: Default::default(),
                genesis_index: 0,
                start_index: None,
            },
            err: Some(BlockError::DuplicateRelatedTransaction.into()),
        },
    ];

    tests.into_iter().for_each(|test| {
        test.print();

        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: test.payload.genesis_index as u64,
                    hash: format!("block {}", test.payload.genesis_index),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".into()],
                    // TODO need to make this an i64
                    timestamp_start_index: test.payload.start_index.map(|i| i as u64),
                    ..Default::default()
                },
            },
            "".into(),
        )
        .unwrap();
        todo!()
        // TODO need to fix asserter.
        // let resp = asserter.unwrap().block(&test.block);
    });
}