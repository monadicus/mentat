use super::test_utils::CustomAsserterTest;
use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::MIN_UNIX_EPOCH,
        errors::{BlockError, SearchError},
    },
    types::{
        AccountIdentifier,
        Allow,
        Amount,
        BlockIdentifier,
        BlockTransaction,
        Currency,
        NetworkIdentifier,
        NetworkOptionsResponse,
        NetworkStatusResponse,
        Operation,
        OperationIdentifier,
        OperationStatus,
        Peer,
        SearchTransactionsResponse,
        Transaction,
        TransactionIdentifier,
        Version,
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
        ..Default::default()
    };
    let valid_block_ident = BlockIdentifier {
        hash: "blah".into(),
        index: 100,
    };

    let tests = [
        CustomAsserterTest {
            name: "no transactions",
            payload: Some(Default::default()),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "valid next",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                ..Default::default()
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "invalid next",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(-1),
                ..Default::default()
            }),
            extras: (),
            err: Some(SearchError::NextOffsetInvalid.into()),
        },
        CustomAsserterTest {
            name: "valid count",
            payload: Some(SearchTransactionsResponse {
                total_count: 0,
                ..Default::default()
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "invalid count",
            payload: Some(SearchTransactionsResponse {
                total_count: -1,
                ..Default::default()
            }),
            extras: (),
            err: Some(SearchError::TotalCountInvalid.into()),
        },
        CustomAsserterTest {
            name: "valid next + transaction",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![Some(BlockTransaction {
                    block_identifier: Some(valid_block_ident.clone()),
                    transaction: Some(valid_transaction.clone()),
                })],
                ..Default::default()
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "valid next + invalid blockIdentifier",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![Some(BlockTransaction {
                    block_identifier: Default::default(),
                    transaction: Some(valid_transaction),
                })],
                ..Default::default()
            }),
            extras: (),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        CustomAsserterTest {
            name: "valid next + invalid transaction",
            payload: Some(SearchTransactionsResponse {
                next_offset: Some(1),
                transactions: vec![Some(BlockTransaction {
                    block_identifier: Some(valid_block_ident),
                    transaction: Default::default(),
                })],
                ..Default::default()
            }),
            extras: (),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
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

    CustomAsserterTest::custom_asserter_tests(
        &tests,
        asserter,
        Asserter::search_transaction_response,
    );
}
