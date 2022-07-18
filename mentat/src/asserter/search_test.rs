use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        asserter_tools::Asserter,
        block::MIN_UNIX_EPOCH,
        errors::{AsserterError, BlockError},
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

struct SearchTxsRespTest {
    resp: SearchTransactionsResponse,
    err: Option<AsserterError>,
}

#[test]
fn test_search_transactions_response() {
    let valid_account = Some(AccountIdentifier {
        address: "test".to_string(),
        sub_account: None,
        metadata: Default::default(),
    });
    let valid_amount = Some(Amount {
        value: "1000".to_string(),
        currency: Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        },
        metadata: Default::default(),
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
                account: valid_account,
                amount: valid_amount,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let valid_block_ident = BlockIdentifier {
        hash: "blah".to_string(),
        index: 100,
    };

    let tests: IndexMap<&str, SearchTxsRespTest> = indexmap!(
      "no transactions" => SearchTxsRespTest {
        resp: Default::default(),
        err: None,
      },
      "valid next" => SearchTxsRespTest {
        resp: SearchTransactionsResponse {
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
      "valid count" => SearchTxsRespTest {
        resp: SearchTransactionsResponse {
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
      "valid next + transaction" => SearchTxsRespTest {
        resp: SearchTransactionsResponse {
          next_offset: Some(1),
          transactions: vec![
            BlockTransaction { block_identifier: valid_block_ident.clone(), transaction: valid_transaction.clone() },
          ],
          ..Default::default()
        },
        err: None,
      },
      "valid next + invalid blockIdentifier" => SearchTxsRespTest {
        resp: SearchTransactionsResponse {
          next_offset: Some(1),
          transactions: vec![
            BlockTransaction { block_identifier: Default::default(), transaction: valid_transaction },
          ],
          ..Default::default()
        },
        err: Some(BlockError::BlockIdentifierHashMissing.into()),
      },
      "valid next + invalid transaction" => SearchTxsRespTest {
        resp: SearchTransactionsResponse {
          next_offset: Some(1),
          transactions: vec![
            BlockTransaction { block_identifier: valid_block_ident, transaction: Default::default() },
          ],
          ..Default::default()
        },
        err: Some(BlockError::BlockIdentifierHashMissing.into()),
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
            Default::default(),
        );
        assert!(asserter.is_err());
    });
}
