use super::*;

fn simple_asserter_configuration(allowed_status: Vec<Option<OperationStatus>>) -> Asserter {
    Asserter::new_client_with_options(
        Some(NetworkIdentifier {
            blockchain: "bitcoin".to_string(),
            network: "mainnent".to_string(),
            ..Default::default()
        }),
        Some(BlockIdentifier {
            hash: "block 0".to_string(),
            index: 0,
        }),
        vec!["Transfer".to_string()],
        allowed_status,
        Vec::new(),
        None,
        Validations {
            enabled: false,
            ..Default::default()
        },
    )
    .unwrap()
}

struct BalanceChangesTest {
    pub(crate) block: Block,
    pub(crate) orphan: bool,
    pub(crate) changes: Vec<BalanceChange>,
}

#[test]
fn test_balance_changes() {
    let currency = Currency {
        symbol: "blah".to_string(),
        decimals: 2,
        ..Default::default()
    };
    let recipient = Some(AccountIdentifier {
        address: "acct1".to_string(),
        ..Default::default()
    });
    let recipient_amount = Some(Amount {
        value: "100".to_string(),
        currency: currency.clone(),
        ..Default::default()
    });
    let empty_account_and_amount = Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            network_index: None,
        },
        type_: "Transfer".to_string(),
        status: Some("Success".to_string()),
        ..Default::default()
    };
    let empty_amount = Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            network_index: None,
        },
        type_: "Transfer".to_string(),
        status: Some("Success".to_string()),
        account: recipient.clone(),
        ..Default::default()
    };
    let recipient_operation = Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            network_index: None,
        },
        type_: "Transfer".to_string(),
        status: Some("Success".to_string()),
        account: recipient.clone(),
        amount: recipient_amount.clone(),
        ..Default::default()
    };
    let recipient_failure_operation = Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            network_index: None,
        },
        type_: "Transfer".to_string(),
        status: Some("Failure".to_string()),
        account: recipient.clone(),
        amount: recipient_amount,
        ..Default::default()
    };
    let recipient_transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "tx1".to_string(),
        },
        operations: vec![
            empty_account_and_amount,
            empty_amount,
            recipient_operation.clone(),
            recipient_failure_operation,
        ],
        ..Default::default()
    };
    let default_status = vec![
        Some(OperationStatus {
            status: "Success".to_string(),
            successful: true,
        }),
        Some(OperationStatus {
            status: "Failure".to_string(),
            successful: false,
        }),
    ];

    let tests = vec![
        CustomParserTest {
            name: "simple block",
            payload: BalanceChangesTest {
                block: Block {
                    block_identifier: BlockIdentifier {
                        hash: "1".into(),
                        index: 1,
                    },
                    parent_block_identifier: BlockIdentifier {
                        hash: "0".into(),
                        index: 0,
                    },
                    transactions: vec![recipient_transaction.clone()],
                    timestamp: MIN_UNIX_EPOCH + 1,
                    ..Default::default()
                },
                orphan: false,
                changes: vec![BalanceChange {
                    account: recipient,
                    currency: Some(currency),
                    block: BlockIdentifier {
                        hash: "1".into(),
                        index: 1,
                    },
                    difference: "100".into(),
                }],
            },
            asserter_extras: default_status.clone(),
            parser_extras: None,
            err: None,
        },
        CustomParserTest {
            name: "simple block account exempt",
            payload: BalanceChangesTest {
                block: Block {
                    block_identifier: BlockIdentifier {
                        hash: "1".into(),
                        index: 1,
                    },
                    parent_block_identifier: BlockIdentifier {
                        hash: "0".into(),
                        index: 0,
                    },
                    transactions: vec![recipient_transaction],
                    timestamp: MIN_UNIX_EPOCH + 1,
                    ..Default::default()
                },
                orphan: false,
                changes: vec![],
            },
            asserter_extras: default_status,
            parser_extras: Some(|op: &Operation| {
                hash(op.account.as_ref()) == hash(recipient_operation.clone().account.as_ref())
            }),
            err: None,
        },
    ];

    CustomParserTest::run(
        tests,
        simple_asserter_configuration,
        Parser::new,
        |parser, payload| {
            let res = parser
                .balance_changes((), &payload.block, payload.orphan)
                .unwrap();
            if res != payload.changes {
                println!(
                    "test returned wrong value: `{:?}` != `{:?}`",
                    payload.changes, res
                );
                false
            } else {
                println!("ok!");
                true
            }
        },
    );
}
