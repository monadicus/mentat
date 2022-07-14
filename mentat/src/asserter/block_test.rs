use std::path::PathBuf;

use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::{
            account_identifier, amount, block_identifier, operation_identifier, MIN_UNIX_EPOCH,
        },
        errors::{AsserterError, BlockError},
    },
    types::{
        AccountIdentifier, Allow, Amount, BlockIdentifier, Currency, NetworkIdentifier,
        NetworkOptionsResponse, NetworkStatusResponse, Operation, OperationIdentifier,
        OperationStatus, Peer, SubAccountIdentifier, Version,
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
        // "nil identifier" => BlockIdentTest {
        //     ident: None,
        //     err: Some(BlockError::BlockIdentifierIsNil.into())
        // },
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

    tests.iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = block_identifier(&test.ident);

        if let Err(err) = resp {
            assert_eq!(
                Some(err.to_string()),
                test.err.as_ref().map(|e| e.to_string()),
            )
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
        // "nil currency" => AmountTest {
        // 	amt: Amount {
        // 		value: "-100000".to_string(),
        // 		currency: None,
        // 		metadata: Default::default(),
        // 	},
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

    tests.iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = amount(test.amt.as_ref());

        if let Err(err) = resp {
            assert_eq!(
                Some(err.to_string()),
                test.err.as_ref().map(|e| e.to_string()),
            )
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
        // "invalid identifier with network index" => OperationIdentTest {
        // 	ident: OperationIdentifier {
        // 		index: 0,
        // 		network_index: Some(invalid_network_index),
        // 	},
        // 	index: 0,
        // 	err: Some(BlockError::OperationIdentifierNetworkIndexInvalid.into())
        // },
    );

    tests.iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = operation_identifier(&test.ident, test.index);

        if let Err(err) = resp {
            assert_eq!(
                Some(err.to_string()),
                test.err.as_ref().map(|e| e.to_string()),
            )
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

    tests.iter().for_each(|(name, test)| {
        println!("test: {name}");

        let resp = account_identifier(test.ident.as_ref());

        if let Err(err) = resp {
            assert_eq!(
                Some(err.to_string()),
                test.err.as_ref().map(|e| e.to_string()),
            )
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

    tests.iter().for_each(|(name, test)| {
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
            test.validation_file_path.clone(),
        );
		assert!(asserter.is_err());

        let resp = asserter.unwrap().operations(&test.operations, test.construction);

        if let Err(err) = resp {
            assert_eq!(
                Some(err.to_string()),
                test.err.as_ref().map(|e| e.to_string()),
            )
        } else {
            assert_eq!(None, test.err);
        }
    });
}
