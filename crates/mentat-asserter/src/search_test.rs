use super::*;

#[test]
fn test_search_transactions_response() {
    let valid_transaction = Some(UncheckedTransaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: vec![
            Some(UncheckedOperation {
                operation_identifier: Some(UncheckedOperationIdentifier {
                    index: 0,
                    network_index: None,
                }),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account(),
                amount: valid_amount(),
                ..Default::default()
            }),
            Some(UncheckedOperation {
                operation_identifier: Some(UncheckedOperationIdentifier {
                    index: 1,
                    network_index: None,
                }),
                related_operations: vec![Some(UncheckedOperationIdentifier {
                    index: 0,
                    network_index: None,
                })],
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account(),
                amount: valid_amount(),
                ..Default::default()
            }),
        ],
        ..Default::default()
    });

    let tests = vec![
        TestCase {
            name: "no transactions",
            payload: Default::default(),
            criteria: None,
        },
        TestCase {
            name: "valid next",
            payload: UncheckedSearchTransactionsResponse {
                next_offset: Some(1),
                ..Default::default()
            },
            criteria: None,
        },
        TestCase {
            name: "invalid next",
            payload: UncheckedSearchTransactionsResponse {
                next_offset: Some(-1),
                ..Default::default()
            },
            criteria: Some(SearchError::NextOffsetInvalid.into()),
        },
        TestCase {
            name: "valid count",
            payload: UncheckedSearchTransactionsResponse {
                total_count: 0,
                ..Default::default()
            },
            criteria: None,
        },
        TestCase {
            name: "invalid count",
            payload: UncheckedSearchTransactionsResponse {
                total_count: -1,
                ..Default::default()
            },
            criteria: Some(SearchError::TotalCountInvalid.into()),
        },
        TestCase {
            name: "valid next + transaction",
            payload: UncheckedSearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![Some(UncheckedBlockTransaction {
                    block_identifier: valid_block_identifier(),
                    transaction: valid_transaction.clone(),
                })],
                ..Default::default()
            },
            criteria: None,
        },
        TestCase {
            name: "valid next + invalid blockIdentifier",
            payload: UncheckedSearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![Some(UncheckedBlockTransaction {
                    block_identifier: Some(Default::default()),
                    transaction: valid_transaction,
                })],
                ..Default::default()
            },
            criteria: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        TestCase {
            name: "valid next + invalid transaction",
            payload: UncheckedSearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![Some(UncheckedBlockTransaction {
                    block_identifier: valid_block_identifier(),
                    transaction: Some(Default::default()),
                })],
                ..Default::default()
            },
            criteria: Some(BlockError::TxIdentifierIsNil.into()),
        },
    ];

    let asserter = Asserter::new_client_with_responses(
        Some(NetworkIdentifier {
            blockchain: "HELLO".into(),
            network: "WORLD".into(),
            sub_network_identifier: None,
        }),
        Some(UncheckedNetworkStatusResponse {
            current_block_identifier: Some(UncheckedBlockIdentifier {
                index: 100,
                hash: "block 100".into(),
            }),
            current_block_timestamp: MIN_UNIX_EPOCH + 1,
            genesis_block_identifier: Some(UncheckedBlockIdentifier {
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
        Some(UncheckedNetworkOptionsResponse {
            version: Some(Version {
                rosetta_version: "1.4.0".into(),
                node_version: "1.0".into(),
                middleware_version: None,
                metadata: Default::default(),
            }),
            allow: Some(UncheckedAllow {
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

    TestCase::run_err_match(tests, |t| asserter.search_transaction_response(Some(&t)));
}
