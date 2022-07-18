use std::path::PathBuf;

use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::{
            account_identifier,
            amount,
            block_identifier,
            operation_identifier,
            MAX_UNIX_EPOCH,
            MIN_UNIX_EPOCH,
        },
        errors::{AsserterError, BlockError},
    },
    types::{
        AccountIdentifier,
        Allow,
        Amount,
        Block,
        BlockIdentifier,
        Currency,
        Direction,
        NetworkIdentifier,
        NetworkOptionsResponse,
        NetworkStatusResponse,
        Operation,
        OperationIdentifier,
        OperationStatus,
        Peer,
        RelatedTransaction,
        SubAccountIdentifier,
        Transaction,
        TransactionIdentifier,
        Version,
    },
};

struct BlockIdentTest {
    ident: BlockIdentifier,
    err: Option<AsserterError>,
}

#[test]
fn test_block_identifier() {
    let tests: IndexMap<&str, BlockIdentTest> = indexmap!(
        "valid identifier" => BlockIdentTest {
            ident: BlockIdentifier {
                index: 1,
                hash: "block 1".to_string()
            },
            err: None
        },
        // TODO allow None BlockIdent
        // "nil identifier" => BlockIdentTest {
        //     ident: None,
        //     err: Some(BlockError::BlockIdentifierIsNil.into())
        // },
        // TODO make block ident index i64
        // "invalid index" => BlockIdentTest {
        //     ident: BlockIdentifier {
        // 		index: -1,
        // 		hash: "block 1".to_string()
        // 	},
        //     err: Some(BlockError::BlockIdentifierIndexIsNeg.into())
        // },
        "invalid hash" => BlockIdentTest {
            ident: BlockIdentifier {
                index: 1,
                hash: String::new(),
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        }
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = block_identifier(&test.ident);

        if let Err(err) = resp {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct AmountTest {
    amt: Option<Amount>,
    err: Option<AsserterError>,
}

#[test]
fn test_amount() {
    let tests: IndexMap<&str, AmountTest> = indexmap!(
        "valid amount" => AmountTest {
            amt: Some(Amount {
                value: "100000".to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: 1,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: None
        },
        "valid amount no decimals" => AmountTest {
            amt: Some(Amount {
                value: "100000".to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: Default::default(),
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: None
        },
        "valid negative amount" => AmountTest {
            amt: Some(Amount {
                value: "-100000".to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: 1,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }),
            err: None
        },
        "nil amount" => AmountTest {
            amt: None,
            err: Some(BlockError::AmountValueMissing.into())
        },
        // Allow None currency
        // "nil currency" => AmountTest {
        // 	amt: Some(Amount {
        // 		value: "-100000".to_string(),
        // 		currency: None,
        // 		metadata: Default::default(),
        // 	}),
        // 	err: Some(BlockError::AmountCurrencyIsNil.into())
        // },
        "invalid non number" => AmountTest {
            amt: Some(Amount {
                value: "blah".to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
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
        "invalid integer format" => AmountTest {
            amt: Some(Amount {
                value: "1.0".to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
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
        "invalid non-integer" => AmountTest {
            amt: Some(Amount {
                value: "1.1".to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
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
        "invalid symbol" => AmountTest {
            amt: Some(Amount {
                value: "11".to_string(),
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
        // "invalid decimals" => AmountTest {
        // 	amt: Amount {
        // 		value: "1.0".to_string(),
        // 		currency: Currency {
        // 			symbol: "BTC".to_string(),
        // 			decimals: -1,
        // 			metadata: Default::default(),
        // 		},
        // 		metadata: Default::default(),
        // 	},
        // 	err: Some(BlockError::AmountCurrencyHasNegDecimals.into())
        // },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = amount(test.amt.as_ref());

        if let Err(err) = resp {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct OperationIdentTest {
    ident: OperationIdentifier,
    index: i64,
    err: Option<AsserterError>,
}

#[test]
fn test_operation_identifier() {
    let valid_network_index = 1;
    // TODO make network indexs i64
    // let invalid_network_index = -1;

    let tests: IndexMap<&str, OperationIdentTest> = indexmap!(
        "valid identifier" => OperationIdentTest {
            ident: OperationIdentifier {
                index: 0,
                network_index: None
            },
            index: 0,
            err: None
        },
        // TODO allow None OperationIdentifier
        // "nil identifier" => OperationIdentTest {
        // 	ident: None,
        // 	index: 0,
        // 	err: Some(BlockError::OperationIdentifierIndexIsNil.into()),
        // },
        "out-of-order index" => OperationIdentTest {
            ident: OperationIdentifier {
                index: 0,
                network_index: None
            },
            index: 1,
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into())
        },
        "valid identifier with network index" => OperationIdentTest {
            ident: OperationIdentifier {
                index: 0,
                network_index: Some(valid_network_index),
            },
            index: 0,
            err: None,
        },
        // TODO see invalid_network_index defined above
        // "invalid identifier with network index" => OperationIdentTest {
        // 	ident: OperationIdentifier {
        // 		index: 0,
        // 		network_index: Some(invalid_network_index),
        // 	},
        // 	index: 0,
        // 	err: Some(BlockError::OperationIdentifierNetworkIndexInvalid.into())
        // },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = operation_identifier(&test.ident, test.index);

        if let Err(err) = resp {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct AccountIdentTest {
    ident: Option<AccountIdentifier>,
    err: Option<AsserterError>,
}

#[test]
fn test_account_identifier() {
    let tests: IndexMap<&str, AccountIdentTest> = indexmap!(
        "valid identifier" => AccountIdentTest {
            ident: Some(AccountIdentifier {
                address: "acct1".to_string(),
                sub_account: None,
                metadata: Default::default()
            }),
            err: None
        },
        "invalid identifier" => AccountIdentTest {
            ident: Some(AccountIdentifier {
                address: String::new(),
                sub_account: None,
                metadata: Default::default()
            }),
            err: Some(BlockError::AccountAddrMissing.into())
        },
        "valid identifier with subaccount" => AccountIdentTest {
            ident: Some(AccountIdentifier {
                address: "acct1".to_string(),
                sub_account: Some(SubAccountIdentifier {
                    address: "acct2".to_string(),
                    metadata: Default::default()
                }),
                metadata: Default::default()
            }),
            err: None
        },
        "invalid identifier with subaccount" => AccountIdentTest {
            ident: Some(AccountIdentifier {
                address: "acct1".to_string(),
                sub_account: Some(SubAccountIdentifier {
                    address: String::new(),
                    metadata: Default::default()
                }),
                metadata: Default::default()
            }),
            err: Some(BlockError::AccountSubAccountAddrMissing.into())
        }
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = account_identifier(test.ident.as_ref());

        if let Err(err) = resp {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct OperationValidationsTest {
    operations: Vec<Operation>,
    validation_file_path: PathBuf,
    construction: bool,
    err: Option<AsserterError>,
}

#[test]
fn test_operation_validations() {
    let valid_deposit_amt = Amount {
        value: "1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let valid_withdraw_amt = Amount {
        value: "-1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let valid_fee_amt = Amount {
        value: "-100".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let invalid_fee_amt = Amount {
        value: "100".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    };
    let valid_account = AccountIdentifier {
        address: "test".to_string(),
        sub_account: None,
        metadata: Default::default(),
    };

    let tests: IndexMap<&str, OperationValidationsTest> = indexmap!(
        "valid operations based on validation file" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_withdraw_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 2, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_fee_amt.clone()),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_balanced.json"),
            construction: false,
            err: None,
        },
        "throw error on missing fee operation" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_withdraw_amt),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_balanced.json"),
            construction: false,
            err: Some(BlockError::FeeCountMismatch.into()),
        },
        "throw error on missing payment operation" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_fee_amt.clone()),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_balanced.json"),
            construction: false,
            err: Some(BlockError::PaymentCountMismatch.into()),
        },
        "throw error on payment amount not balancing" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(Amount {
                        value: "-2000".to_string(),
                        currency: Currency {
                            symbol: "BTC".to_string(),
                            decimals: 8,
                            metadata: Default::default()
                        },
                        metadata: Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 2, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_fee_amt.clone()),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_balanced.json"),
            construction: false,
            err: Some(BlockError::PaymentAmountNotBalancing.into()),
        },
        "valid operations based on validation file - unbalanced" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(Amount {
                        value: "-2000".to_string(),
                        currency: Currency {
                            symbol: "BTC".to_string(),
                            decimals: 8,
                            metadata: Default::default()
                        },
                        metadata: Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 2, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_fee_amt.clone()),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_unbalanced.json"),
            construction: false,
            err: None,
        },
        "fee operation shouldn't contain related_operation key" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(Amount {
                        value: "-2000".to_string(),
                        currency: Currency {
                            symbol: "BTC".to_string(),
                            decimals: 8,
                            metadata: Default::default()
                        },
                        metadata: Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 2, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_fee_amt.clone()),
                    related_operations: Some(vec![
                        OperationIdentifier { index: 0, network_index: None }
                    ]),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_unbalanced.json"),
            construction: false,
            err: Some(BlockError::RelatedOperationInFeeNotAllowed.into()),
        },
        "fee amount is non-negative" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt.clone()),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(Amount {
                        value: "-2000".to_string(),
                        currency: Currency {
                            symbol: "BTC".to_string(),
                            decimals: 8,
                            metadata: Default::default()
                        },
                        metadata: Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 2, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(invalid_fee_amt),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_unbalanced.json"),
            construction: false,
            err: Some(BlockError::FeeAmountNotNegative.into()),
        },
        "fee amount is negative as expected" => OperationValidationsTest {
            operations: vec![
                Operation {
                    operation_identifier: OperationIdentifier { index: 0, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(valid_deposit_amt),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 1, network_index: None },
                    type_: "PAYMENT".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account.clone()),
                    amount: Some(Amount {
                        value: "-2000".to_string(),
                        currency: Currency {
                            symbol: "BTC".to_string(),
                            decimals: 8,
                            metadata: Default::default()
                        },
                        metadata: Default::default()
                    }),
                    ..Default::default()
                },
                Operation {
                    operation_identifier: OperationIdentifier { index: 2, network_index: None },
                    type_: "FEE".to_string(),
                    status: Some("SUCCESS".to_string()),
                    account: Some(valid_account),
                    amount: Some(valid_fee_amt),
                    related_operations: Some(vec![
                        OperationIdentifier { index: 0, network_index: None }
                    ]),
                    ..Default::default()
                },
            ],
            validation_file_path: PathBuf::from("data/validation_fee_and_payment_unbalanced.json"),
            construction: false,
            err: None,
        },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".to_string(),
                network: "world".to_string(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".to_string(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: 0,
                    hash: "block 0".to_string(),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".to_string(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".to_string(),
                    node_version: "1.0".to_string(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".to_string(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".to_string(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".to_string(), "FEE".to_string()],
                    ..Default::default()
                },
            },
            test.validation_file_path,
        );
        assert!(asserter.is_err());

        // let resp = asserter.unwrap().operations(&test.operations,
        // test.construction);
    });
}

struct OperationTest {
    operation: Operation,
    index: i64,
    succesful: bool,
    construction: bool,
    err: Option<AsserterError>,
}

#[test]
fn test_operation() {
    let valid_amount = Some(Amount {
        value: "1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    });
    let valid_account = Some(AccountIdentifier {
        address: "test".to_string(),
        ..Default::default()
    });

    let tests: IndexMap<&str, OperationTest> = indexmap!(
        "valid operation" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            index: 1,
            succesful: true,
            construction: false,
            err: None
        },
        "valid operation no account" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            index: 1,
            succesful: true,
            construction: false,
            err: None
        },
        // TODO allow nill operations
        // "nil operation" => OperationTest {
        //     operation: None,
        //     index: 1,
        //     successful: false,
        //     construction: false,
        //     err: Some(BlockError::OperationIsNil.into()),
        // },
        "invalid operation no account" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: false,
            err: Some(BlockError::OperationIsNil.into()),
        },
        "invalid operation empty account" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: Some(AccountIdentifier::default()),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: false,
            err: Some(BlockError::AccountAddrMissing.into()),
        },
        "invalid operation invalid index" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                ..Default::default()
            },
            index: 2,
            succesful: false,
            construction: false,
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        "invalid operation invalid type" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "STAKE".to_string(),
                status: Some("SUCCESS".to_string()),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: false,
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        "unsuccessful operation" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("FAILURE".to_string()),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: false,
            err: None,
        },
        "invalid operation invalid status" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("DEFERRED".to_string()),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: false,
            err: Some(BlockError::OperationStatusInvalid.into()),
        },
        "valid construction operation" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: true,
            err: None,
        },
        "valid construction operation (empty status)" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some(String::new()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: true,
            err: None,
        },
        "invalid construction operation" => OperationTest {
            operation: Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            },
            index: 1,
            succesful: false,
            construction: true,
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");
        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".to_string(),
                network: "world".to_string(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".to_string(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: 0,
                    hash: "block 0".to_string(),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".to_string(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".to_string(),
                    node_version: "1.0".to_string(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".to_string(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".to_string(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".to_string()],
                    ..Default::default()
                },
            },
            "".into(),
        );
        assert!(asserter.is_err());

        // let resp = asserter.unwrap().operation(&test.operation, test.index,
        // test.construction);
    });
}

struct BlockTest {
    block: Block,
    // TODO consider making this an options
    validation_file_path: PathBuf,
    genesis_index: i64,
    start_index: Option<i64>,
    err: Option<AsserterError>,
}

#[test]
fn test_block() {
    let genesis_ident = BlockIdentifier {
        hash: "gen".to_string(),
        index: 0,
    };
    let valid_block_ident = BlockIdentifier {
        hash: "blah".to_string(),
        index: 100,
    };
    let valid_parent_block_ident = BlockIdentifier {
        hash: "blah parent".to_string(),
        index: 99,
    };
    let valid_amount = Some(Amount {
        value: "1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
    });
    let valid_account = Some(AccountIdentifier {
        address: "test".to_string(),
        ..Default::default()
    });
    let valid_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
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
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        related_transactions: Some(vec![RelatedTransaction {
            network_identifier: Some(NetworkIdentifier {
                blockchain: "hello".to_string(),
                network: "world".to_string(),
                sub_network_identifier: None,
            }),
            transaction_identifier: TransactionIdentifier {
                hash: "blah".to_string(),
            },
            direction: Direction::Forward,
        }]),
        metadata: Default::default(),
    };
    let related_to_self_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
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
            type_: "PAYMENT".to_string(),
            status: Some("SUCCESS".to_string()),
            account: valid_account.clone(),
            amount: valid_amount.clone(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let out_of_order_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
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
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let related_to_later_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
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
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
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
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let related_duplicate_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
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
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let related_missing_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(Vec::new()),
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 1,
                    network_index: None,
                },
                related_operations: Some(Vec::new()),
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
                amount: valid_amount.clone(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    // TODO allow arbitrary directions
    // let invalid_related_transaction = Transaction {
    //     transaction_identifier: TransactionIdentifier {
    //         hash: "blah".to_string(),
    //     },
    //     operations: vec![
    //         Operation {
    //             operation_identifier: OperationIdentifier {
    //                 index: 0,
    //                 network_index: None,
    //             },
    //             type_: "PAYMENT".to_string(),
    //             status: Some("SUCCESS".to_string()),
    //             account: valid_account.clone(),
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
    //             type_: "PAYMENT".to_string(),
    //             status: Some("SUCCESS".to_string()),
    //             account: valid_account.clone(),
    //             amount: valid_amount.clone(),
    //             ..Default::default()
    //         },
    //     ],
    //     related_transactions: Some(vec![RelatedTransaction {
    //         network_identifier: Some(NetworkIdentifier {
    //             blockchain: "hello".to_string(),
    //             network: "world".to_string(),
    //             sub_network_identifier: None,
    //         }),
    //         transaction_identifier: TransactionIdentifier {
    //             hash: "blah".to_string(),
    //         },
    //         direction: "blah",
    //     }]),
    //     ..Default::default()
    // };
    let duplicated_related_transactions = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "blah".to_string(),
        },
        operations: vec![
            Operation {
                operation_identifier: OperationIdentifier {
                    index: 0,
                    network_index: None,
                },
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account.clone(),
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
                type_: "PAYMENT".to_string(),
                status: Some("SUCCESS".to_string()),
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            },
        ],
        related_transactions: Some(vec![
            RelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "hello".to_string(),
                    network: "world".to_string(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: TransactionIdentifier {
                    hash: "blah".to_string(),
                },
                direction: Direction::Forward,
            },
            RelatedTransaction {
                network_identifier: Some(NetworkIdentifier {
                    blockchain: "hello".to_string(),
                    network: "world".to_string(),
                    sub_network_identifier: None,
                }),
                transaction_identifier: TransactionIdentifier {
                    hash: "blah".to_string(),
                },
                direction: Direction::Forward,
            },
        ]),
        ..Default::default()
    };

    // TODO shouldn't need any cast operations below
    let tests: IndexMap<&str, BlockTest> = indexmap!(
        "valid block" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                parent_block_identifier: valid_parent_block_ident.clone(),
                timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                transactions: vec![valid_transaction.clone()],
                metadata: Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: None,
            err: None,
        },
        "valid block (before start index)" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                parent_block_identifier: valid_parent_block_ident.clone(),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: Some((valid_block_ident.index + 1) as i64),
            err: None,
        },
        "genesis block (without start index)" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                parent_block_identifier: valid_parent_block_ident.clone(),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: valid_block_ident.index as i64,
            start_index: None,
            err: None,
        },
        "genesis block (with start index)" => BlockTest {
            block: Block {
                block_identifier: genesis_ident.clone(),
                parent_block_identifier: genesis_ident.clone(),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: genesis_ident.index as i64,
            start_index: Some((genesis_ident.index + 1) as i64),
            err: None,
        },
        "invalid genesis block (with start index)" => BlockTest {
            block: Block {
                block_identifier: genesis_ident.clone(),
                parent_block_identifier: genesis_ident.clone(),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: genesis_ident.index as i64,
            start_index: Some(genesis_ident.index as i64),
            err: Some(BlockError::TimestampBeforeMin.into()),
        },
        "out of order transaction operations" => BlockTest {
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
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        "related to self transaction operations" => BlockTest {
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
            err: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        "related to later transaction operations" => BlockTest {
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
            err: Some(BlockError::RelatedOperationIndexOutOfOrder.into()),
        },
        "duplicate related transaction operations" => BlockTest {
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
            err: Some(BlockError::RelatedOperationIndexDuplicate.into()),
        },
        "missing related transaction operations" => BlockTest {
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
            err: Some(BlockError::RelatedOperationMissing.into()),
        },
        // TODO allow None block
        // "nil block" => BlockTest {
        //     block: None,
        //     validation_file_path:  Default::default(),
        //     genesis_index: 0,
        //     start_index: None,
        //     err: Some(BlockError::BlockIsNil.into()),
        // },
        // TODO allow block ident to be None
        // "nil block hash" => BlockTest {
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
        //     err: Some(BlockError::BlockIdentifierIsNil.into()),
        // },
        "invalid block hash" => BlockTest {
            block: Block {
                parent_block_identifier: valid_parent_block_ident.clone(),
                timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: None,
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        "block previous hash missing" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: None,
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        "invalid parent block index" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                parent_block_identifier: BlockIdentifier { index: valid_block_ident.index , hash: valid_parent_block_ident.hash.clone() },
                timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: None,
            err: Some(BlockError::BlockIndexPrecedesParentBlockIndex.into()),
        },
        "invalid parent block hash" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                parent_block_identifier: BlockIdentifier { index: valid_parent_block_ident.index , hash: valid_block_ident.hash.clone() },
                timestamp: (MIN_UNIX_EPOCH + 1) as u64,
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: None,
            err: Some(BlockError::BlockHashEqualsParentBlockHash.into()),
        },
        "invalid block timestamp less than MinUnixEpoch" => BlockTest {
            block: Block {
                block_identifier: valid_block_ident.clone(),
                parent_block_identifier: valid_parent_block_ident.clone(),
                transactions: vec![valid_transaction.clone()],
                ..Default::default()
            },
            validation_file_path: Default::default(),
            genesis_index: 0,
            start_index: None,
            err: Some(BlockError::TimestampBeforeMin.into()),
        },
        "invalid block timestamp greater than MaxUnixEpoch" => BlockTest {
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
            err: Some(BlockError::TimestampAfterMax.into()),
        },
        "invalid block transaction" => BlockTest {
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
            err: Some(BlockError::TxIdentifierIsNil.into()),
        },
        // TODO see invalid_related_transaction defined above
        // "invalid related transaction" => BlockTest {
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
        //     err: Some(BlockError::InvalidDirection.into()),
        // },
        // TODO see invalid_related_transaction defined above
        "invalid related transaction" => BlockTest {
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
            err: Some(BlockError::DuplicateRelatedTransaction.into()),
        },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");
        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".to_string(),
                network: "world".to_string(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".to_string(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: test.genesis_index as u64,
                    hash: format!("block {}", test.genesis_index),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".to_string(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".to_string(),
                    node_version: "1.0".to_string(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".to_string(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".to_string(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".to_string()],
                    // TODO need to make this an i64
                    timestamp_start_index: test.start_index.map(|i| i as u64),
                    ..Default::default()
                },
            },
            "".into(),
        );
        assert!(asserter.is_err());

        // TODO need to fix asserter.
        // let resp = asserter.unwrap().block(&test.block);
    });
}
