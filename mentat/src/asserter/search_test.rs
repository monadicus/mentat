use super::test_utils::AsserterTest;
use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::MIN_UNIX_EPOCH,
        errors::{BlockError, SearchError},
    },
    types::{
        AccountIdentifier, Allow, Amount, BlockIdentifier, BlockTransaction, Currency,
        NetworkIdentifier, NetworkOptionsResponse, NetworkStatusResponse, Operation,
        OperationIdentifier, OperationStatus, Peer, SearchTransactionsResponse, Transaction,
        TransactionIdentifier, Version,
    },
};

#[test]
fn test_search_transactions_response() {
    let valid_account = Some(AccountIdentifier {
        address: "test".into(),
        sub_account: None,
        metadata: Default::default(),
    });
    let valid_amount = Some(Amount {
        value: "1000".into(),
        currency: Some(Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        }),
        metadata: Default::default(),
    });
    let valid_transaction = Transaction {
        transaction_identifier: Some(TransactionIdentifier {
            hash: "blah".into(),
        }),
        operations: Some(vec![
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
                related_operations: Some(vec![Some(OperationIdentifier {
                    index: 0,
                    network_index: None,
                })]),
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            }),
        ]),
        ..Default::default()
    };
    let valid_block_ident = BlockIdentifier {
        hash: "blah".into(),
        index: 100,
    };

    let tests = [
        AsserterTest {
            name: "no transactions",
            payload: Default::default(),
            err: None,
        },
        AsserterTest {
            name: "valid next",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid next",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(-1),
                ..Default::default()
            }),
            err: Some(SearchError::NextOffsetInvalid.into()),
        },
        AsserterTest {
            name: "valid count",
            payload: Some(SearchTransactionsResponse {
                total_count: 0,
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid count",
            payload: Some(SearchTransactionsResponse {
                total_count: -1,
                ..Default::default()
            }),
            err: Some(SearchError::TotalCountInvalid.into()),
        },
        AsserterTest {
            name: "valid next + transaction",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: Some(vec![Some(BlockTransaction {
                    block_identifier: Some(valid_block_ident.clone()),
                    transaction: Some(valid_transaction.clone()),
                })]),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "valid next + invalid blockIdentifier",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: Some(vec![Some(BlockTransaction {
                    block_identifier: Default::default(),
                    transaction: Some(valid_transaction),
                })]),
                ..Default::default()
            }),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        AsserterTest {
            name: "valid next + invalid transaction",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: Some(vec![Some(BlockTransaction {
                    block_identifier: Some(valid_block_ident),
                    transaction: Default::default(),
                })]),
                ..Default::default()
            }),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    ];

    tests.into_iter().for_each(|test| {
        println!("test: {test}");

        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
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
                peers: Some(vec![Some(Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                })]),
            },
            NetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(Allow {
                    operation_statuses: Some(vec![
                        Some(OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        }),
                        Some(OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        }),
                    ]),
                    operation_types: Some(vec!["PAYMENT".into()]),
                    ..Default::default()
                }),
            },
            Default::default(),
        )
        .unwrap();

        todo!()
    });
}
