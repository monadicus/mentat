use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::MIN_UNIX_EPOCH,
        errors::{AsserterError, BlockError},
    },
    types::{
        AccountIdentifier, Allow, Amount, BlockIdentifier, BlockTransaction, Currency,
        NetworkIdentifier, NetworkOptionsResponse, NetworkStatusResponse, Operation,
        OperationIdentifier, OperationStatus, Peer, SearchTransactionsResponse, Transaction,
        TransactionIdentifier, Version,
    },
};

use super::test_utils::AsserterTest;

#[test]
fn test_search_transactions_response() {
    let valid_account = Some(AccountIdentifier {
        address: "test".into(),
        sub_account: None,
        metadata: Default::default(),
    });
    let valid_amount = Some(Amount {
        value: "1000".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
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
                type_: "PAYMENT".into(),
                status: Some("SUCCESS".into()),
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            },
        ],
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
            payload: SearchTransactionsResponse {
                next_offset: Some(1),
                ..Default::default()
            },
            err: None,
        },
        // TODO make next offset a i64
        // "invalid next" => SearchTxsRespTest {
        //   resp: SearchTransactionsResponse {
        //     next_offset: Some(-1),
        //     ..Default::default()
        //   },
        //   err: Some(SearchError::NextOffsetInvalid.into()),
        // },
        AsserterTest {
            name: "valid count",
            payload: SearchTransactionsResponse {
                total_count: 0,
                ..Default::default()
            },
            err: None,
        },
        // TODO make total count an i64
        // "invalid count" => SearchTxsRespTest {
        //   resp: SearchTransactionsResponse {
        //     total_count: -1,
        //     ..Default::default()
        //   },
        //   err: Some(SearchError::TotalCountInvalid.into()),
        // },
        AsserterTest {
            name: "valid next + transaction",
            payload: SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![BlockTransaction {
                    block_identifier: valid_block_ident.clone(),
                    transaction: valid_transaction.clone(),
                }],
                ..Default::default()
            },
            err: None,
        },
        AsserterTest {
            name: "valid next + invalid blockIdentifier",
            payload: SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![BlockTransaction {
                    block_identifier: Default::default(),
                    transaction: valid_transaction,
                }],
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        AsserterTest {
            name: "valid next + invalid transaction",
            payload: SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![BlockTransaction {
                    block_identifier: valid_block_ident,
                    transaction: Default::default(),
                }],
                ..Default::default()
            },
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
            Default::default(),
        )
        .unwrap();

        todo!()
    });
}
