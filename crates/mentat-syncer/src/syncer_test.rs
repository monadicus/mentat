#![allow(clippy::missing_docs_in_private_items)]

use std::{
    sync::Arc,
    thread::{sleep, spawn},
    time::Duration,
};

use mentat_types::{
    AccountIdentifier, Amount, Block, BlockIdentifier, Currency, NetworkIdentifier,
    NetworkStatusResponse, Operation, OperationIdentifier, PartialBlockIdentifier, Transaction,
    TransactionIdentifier,
};
use mockall::mock;
use parking_lot::Mutex;

use crate::{
    errors::{SyncerError, SyncerResult},
    syncer::BlockResult,
    types::{ErrorBuf, Handler, Helper, Syncer, DEFAULT_CONCURRENCY},
};

mock! {
    #[allow(clippy::missing_docs_in_private_items)]
    pub Handler {}

    impl Handler for Handler {
        fn block_seen(&self, error_buf: &ErrorBuf, block: &Block) -> SyncerResult<()>;
        fn block_added<'a>(&self, error_buf: &ErrorBuf, block: Option<&'a Block>) -> SyncerResult<()>;
        fn block_removed<'a>(
            &self,
            error_buf: &ErrorBuf,
            block: Option<&'a BlockIdentifier>,
        ) -> SyncerResult<()>;
    }

    impl Clone for Handler {
        fn clone(&self) -> Self;
    }
}

mock! {
    #[allow(clippy::missing_docs_in_private_items)]
    pub Helper {}

    impl Helper for Helper {
        fn network_status(
            &self,
            error_buf: &ErrorBuf,
            network_identifier: &NetworkIdentifier,
        ) -> SyncerResult<NetworkStatusResponse>;

        fn block(
            &self,
            error_buf: &ErrorBuf,
            network_identifier: &NetworkIdentifier,
            partial_block_identifier: &PartialBlockIdentifier,
        ) -> SyncerResult<Option<Block>>;
    }

    impl Clone for Helper {
        fn clone(&self) -> Self;
    }
}

fn network_identifier() -> NetworkIdentifier {
    NetworkIdentifier {
        blockchain: "blah".into(),
        network: "testnet".into(),
        ..Default::default()
    }
}

fn currency() -> Currency {
    Currency {
        symbol: "blah".into(),
        decimals: 2,
        ..Default::default()
    }
}

fn recipient() -> AccountIdentifier {
    AccountIdentifier {
        address: "acct1".into(),
        ..Default::default()
    }
}

pub fn recipient_amount() -> Amount {
    Amount {
        value: "100".to_string(),
        currency: currency(),
        ..Default::default()
    }
}

pub fn recipient_operation() -> Operation {
    Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            ..Default::default()
        },
        type_: "Transfer".to_string(),
        status: Some("Success".into()),
        account: Some(recipient()),
        amount: Some(recipient_amount()),
        ..Default::default()
    }
}

pub fn recipient_failure_operation() -> Operation {
    Operation {
        operation_identifier: OperationIdentifier {
            index: 1,
            ..Default::default()
        },
        type_: "Transfer".to_string(),
        status: Some("Failure".to_string()),
        account: Some(recipient()),
        amount: Some(recipient_amount()),
        ..Default::default()
    }
}

pub fn recipient_transaction() -> Transaction {
    Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "tx1".to_string(),
        },
        operations: vec![recipient_operation(), recipient_failure_operation()],
        ..Default::default()
    }
}

pub fn sender() -> AccountIdentifier {
    AccountIdentifier {
        address: "acct2".to_string(),
        ..Default::default()
    }
}

pub fn sender_amount() -> Amount {
    Amount {
        value: "-100".to_string(),
        currency: currency(),
        ..Default::default()
    }
}

pub fn sender_operation() -> Operation {
    Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            ..Default::default()
        },
        type_: "Transfer".to_string(),
        status: Some("Success".to_string()),
        account: Some(sender()),
        amount: Some(sender_amount()),
        ..Default::default()
    }
}

pub fn sender_transaction() -> Transaction {
    Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "tx2".to_string(),
        },
        operations: vec![sender_operation()],
        ..Default::default()
    }
}

pub fn orphan_genesis() -> Block {
    Block {
        block_identifier: BlockIdentifier {
            hash: "1".to_string(),
            index: 1,
        },
        parent_block_identifier: BlockIdentifier {
            hash: "0a".to_string(),
            index: 0,
        },
        transactions: vec![],
        ..Default::default()
    }
}

pub fn block_sequence() -> [Block; 6] {
    [
        // genesis
        Block {
            block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            ..Default::default()
        },
        Block {
            block_identifier: BlockIdentifier {
                hash: "1".into(),
                index: 1,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            transactions: vec![recipient_transaction()],
            ..Default::default()
        },
        // reorg
        Block {
            block_identifier: BlockIdentifier {
                hash: "2".into(),
                index: 2,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "1a".into(),
                index: 1,
            },
            ..Default::default()
        },
        Block {
            block_identifier: BlockIdentifier {
                hash: "1a".into(),
                index: 1,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            ..Default::default()
        },
        Block {
            block_identifier: BlockIdentifier {
                hash: "3".into(),
                index: 3,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "2".into(),
                index: 2,
            },
            transactions: vec![sender_transaction()],
            ..Default::default()
        },
        // invalid block
        Block {
            block_identifier: BlockIdentifier {
                hash: "5".into(),
                index: 5,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "4".into(),
                index: 4,
            },
            ..Default::default()
        },
    ]
}

fn block_sequence_idx(id: usize) -> Block {
    block_sequence()[id].clone()
}

#[test]
fn test_process_block() {
    let error_buf = Arc::new(Mutex::new(None));
    let mock_helper = MockHelper::new();
    let mock_handler = MockHandler::new();
    let mut syncer = Syncer::builder(network_identifier(), mock_helper, mock_handler).build();
    syncer.genesis_block = Some(block_sequence_idx(0).block_identifier);
    assert!(syncer.past_blocks.is_empty());

    let assert_syncer =
        |syncer: &mut Syncer<MockHandler, MockHelper>, next_index: i64, past_blocks: &[usize]| {
            assert_eq!(syncer.next_index, next_index);
            assert_eq!(
                &syncer.past_blocks,
                &past_blocks
                    .iter()
                    .map(|b| block_sequence_idx(*b).block_identifier)
                    .collect::<Vec<_>>()
            );
            syncer.handler.checkpoint();
        };

    let expect_block_added = |syncer: &mut Syncer<MockHandler, MockHelper>, idx: usize| {
        syncer
            .handler
            .expect_block_added()
            .withf(move |e, g| {
                e.lock().is_none() && matches!(g, Some(g) if **g == block_sequence_idx(idx))
            })
            .return_const(Ok(()))
            .once();
    };

    let expect_block_removed = |syncer: &mut Syncer<MockHandler, MockHelper>, idx: usize| {
        syncer
            .handler
            .expect_block_removed()
            .withf(move |e, b| {
                e.lock().is_none()
                    && matches!(b, Some(b) if **b == block_sequence_idx(idx).block_identifier)
            })
            .return_const(Ok(()))
            .once();
    };

    let process_block = |syncer: &mut Syncer<MockHandler, MockHelper>, block: Option<Block>| {
        syncer.process_block(
            &error_buf,
            Some(BlockResult {
                block,
                ..Default::default()
            }),
        )
    };

    {
        print!("No block exists: ");
        expect_block_added(&mut syncer, 0);
        process_block(&mut syncer, Some(block_sequence_idx(0))).unwrap();
        assert_syncer(&mut syncer, 1, &[0]);
        println!("ok!");
    }

    {
        print!("Orphan genesis: ");
        let err = process_block(&mut syncer, Some(orphan_genesis())).unwrap_err();
        assert_eq!(err.to_string(), "cannot remove genesis block");
        assert_syncer(&mut syncer, 1, &[0]);
        println!("ok!");
    }

    {
        print!("Block exists, no reorg: ");
        expect_block_added(&mut syncer, 1);
        process_block(&mut syncer, Some(block_sequence_idx(1))).unwrap();
        assert_syncer(&mut syncer, 2, &[0, 1]);
        println!("ok!");
    }

    {
        print!("Orphan block: ");
        expect_block_removed(&mut syncer, 1);
        process_block(&mut syncer, Some(block_sequence_idx(2))).unwrap();
        assert_syncer(&mut syncer, 1, &[0]);

        expect_block_added(&mut syncer, 3);
        process_block(&mut syncer, Some(block_sequence_idx(3))).unwrap();
        assert_syncer(&mut syncer, 2, &[0, 3]);

        expect_block_added(&mut syncer, 2);
        process_block(&mut syncer, Some(block_sequence_idx(2))).unwrap();
        assert_syncer(&mut syncer, 3, &[0, 3, 2]);
        println!("ok!");
    }

    {
        print!("Out of order block: ");
        let err = process_block(&mut syncer, Some(block_sequence_idx(5))).unwrap_err();
        assert!(err.to_string().contains("got block 5 instead of 3"));
        assert_syncer(&mut syncer, 3, &[0, 3, 2]);
        println!("ok!");
    }

    {
        print!("Process omitted block: ");
        process_block(&mut syncer, None).unwrap();
        assert_syncer(&mut syncer, 4, &[0, 3, 2]);
        println!("ok!");
    }

    {
        print!("Process nil block result: ");
        let err = syncer.process_block(&error_buf, None).unwrap_err();
        assert_eq!(err, SyncerError::BlockResultNil);
        println!("ok!");
    }

    {
        print!("Process orphan head block result: ");
        expect_block_removed(&mut syncer, 2);
        syncer
            .process_block(
                &error_buf,
                Some(BlockResult {
                    orphaned_head: true,
                    ..Default::default()
                }),
            )
            .unwrap();
        assert_syncer(&mut syncer, 2, &[0, 3]);
        println!("ok!");
    }
}

fn create_blocks(start_index: i64, end_index: i64, add: &str) -> Vec<Option<Block>> {
    (start_index..=end_index)
        .into_iter()
        .map(|i| {
            let parent_index = if i > 0 { i - 1 } else { 0 };
            Some(Block {
                block_identifier: BlockIdentifier {
                    index: i,
                    hash: format!("block {add}{i}"),
                },
                parent_block_identifier: BlockIdentifier {
                    index: parent_index,
                    hash: format!("block {add}{parent_index}"),
                },
                ..Default::default()
            })
        })
        .collect()
}

#[test]
fn test_sync_no_reorg() {
    let error_buf = Arc::new(Mutex::new(None));
    let mock_helper = MockHelper::new();
    let mock_handler = MockHandler::new();
    let mut syncer = Syncer::builder(network_identifier(), mock_helper, mock_handler)
        .with_cancel()
        .max_concurrency(3)
        .build();

    // Tip should be nil before we start syncing
    assert!(syncer.tip.is_none());

    // Force syncer to only get part of the way through the full range
    syncer
        .helper
        .expect_network_status()
        .withf(|e, id| e.lock().is_none() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: 200,
                hash: "block 200".into(),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }))
        .times(2);

    syncer
        .helper
        .expect_network_status()
        .withf(|e, id| e.lock().is_none() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: 1300,
                hash: "block 1300".into(),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }))
        .times(2);

    let mut blocks = create_blocks(0, 1200, "");
    // Create a block gap
    blocks[100] = None;
    blocks[101].as_mut().unwrap().parent_block_identifier =
        blocks[99].as_ref().unwrap().block_identifier.clone();

    for (i, b) in blocks.into_iter().enumerate() {
        syncer
            .helper
            .expect_block()
            .withf(move |e, id, b| {
                e.lock().is_none()
                    && *id == network_identifier()
                    && *b
                        == PartialBlockIdentifier {
                            index: Some(i as i64),
                            ..Default::default()
                        }
            })
            .return_const(Ok(b.clone()))
            .once();

        let b = if let Some(b) = b {
            b
        } else {
            continue;
        };

        let tmp_b = b.clone();
        syncer
            .handler
            .expect_block_seen()
            .withf(move |e, b| e.lock().is_none() && *b == tmp_b)
            .return_const(Ok(()))
            .once();
        syncer
            .handler
            .expect_block_added()
            .withf(move |e, bl| e.lock().is_none() && *bl == Some(&b))
            .return_once(move |_, _| {
                let _id = if i > 200 {
                    BlockIdentifier {
                        index: 1300,
                        hash: "block 1300".into(),
                    }
                } else {
                    BlockIdentifier {
                        index: 200,
                        hash: "block 200".into(),
                    }
                };

                todo!("cant access syncer inside closure");
                // assert_eq!(
                //     Some(&id),
                //     syncer.tip()
                // );
                // Ok(())
            });
    }

    tokio_test::block_on(syncer.sync(&error_buf, -1, 1200)).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.checkpoint();
    syncer.handler.checkpoint();
}

#[test]
fn test_sync_specific_start() {
    let error_buf = Arc::new(Mutex::new(None));
    let mock_helper = MockHelper::new();
    let mock_handler = MockHandler::new();
    let mut syncer = Syncer::builder(network_identifier(), mock_helper, mock_handler)
        .with_cancel()
        .build();

    syncer
        .helper
        .expect_network_status()
        .withf(|e, id| e.lock().is_none() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: 1300,
                hash: "block 1300".into(),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }));

    for b in create_blocks(100, 1200, "") {
        let index = b.as_ref().unwrap().block_identifier.index;
        syncer
            .helper
            .expect_block()
            .withf(move |e, id, b| {
                e.lock().is_none()
                    && *id == network_identifier()
                    && *b
                        == PartialBlockIdentifier {
                            index: Some(index),
                            ..Default::default()
                        }
            })
            .return_const(Ok(b.clone()))
            .once();

        let tmp_b = b.clone().unwrap();
        syncer
            .handler
            .expect_block_seen()
            .withf(move |e, b| e.lock().is_none() && *b == tmp_b)
            .return_const(Ok(()))
            .once();
        let tmp_b = b.clone().unwrap();
        syncer
            .handler
            .expect_block_added()
            .withf(move |e, b| e.lock().is_none() && *b == Some(&tmp_b))
            .return_once(|_, b| {
                if b.unwrap().block_identifier.index == 1100 {
                    todo!("cant access syncer within closure");
                    // assert!(*syncer.concurrency.lock() > DEFAULT_CONCURRENCY)
                }
                Ok(())
            });
    }

    tokio_test::block_on(syncer.sync(&error_buf, 100, 1200)).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.checkpoint();
    syncer.handler.checkpoint();
}

#[test]
fn test_sync_cancel() {
    let error_buf = Arc::new(Mutex::new(None));
    let mock_helper = MockHelper::new();
    let mock_handler = MockHandler::new();
    let mut syncer = Syncer::builder(network_identifier(), mock_helper, mock_handler)
        .with_cancel()
        .build();

    syncer
        .helper
        .expect_network_status()
        .withf(move |e, id| e.lock().is_none() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: 200,
                hash: "block 200".into(),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }))
        .times(2);
    syncer
        .helper
        .expect_network_status()
        .withf(move |e, id| e.lock().is_none() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: 1300,
                hash: "block 1300".into(),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }))
        .times(2);

    for b in create_blocks(0, 1200, "") {
        let index = b.as_ref().unwrap().block_identifier.index;
        syncer
            .helper
            .expect_block()
            .withf(move |e, id, b| {
                e.lock().is_none()
                    && *id == network_identifier()
                    && *b
                        == PartialBlockIdentifier {
                            index: Some(index),
                            ..Default::default()
                        }
            })
            .return_const(Ok(b.clone()))
            .once();

        let tmp_b = b.clone().unwrap();
        syncer
            .handler
            .expect_block_seen()
            .withf(move |e, b| e.lock().is_none() && *b == tmp_b)
            .return_const(Ok(()))
            .once();
        let tmp_b = b.clone().unwrap();
        syncer
            .handler
            .expect_block_added()
            .withf(move |e, b| e.lock().is_none() && *b == Some(&tmp_b))
            .return_const(Ok(()))
            .once();
    }

    let tmp_buf = error_buf.clone();
    let handle = spawn(move || {
        sleep(Duration::from_secs(1));
        *tmp_buf.lock() = Some(SyncerError::Cancelled)
    });
    let err = tokio_test::block_on(syncer.sync(&error_buf, -1, 1200)).unwrap_err();
    assert_eq!(err, SyncerError::Cancelled);
    assert_eq!(0, *syncer.concurrency.lock());
    handle.join().unwrap();
}

#[test]
fn test_sync_reorg() {
    let error_buf = Arc::new(Mutex::new(None));
    let mock_helper = MockHelper::new();
    let mock_handler = MockHandler::new();
    let mut syncer = Syncer::builder(network_identifier(), mock_helper, mock_handler)
        .with_cancel()
        .build();

    syncer
        .helper
        .expect_network_status()
        .withf(move |e, id| e.lock().is_none() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: 1300,
                hash: "block 1300".into(),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }))
        .times(2);

    let blocks = create_blocks(0, 800, "");
    // [0, 800]
    for b in &blocks {
        let index = b.as_ref().unwrap().block_identifier.index;
        syncer
            .helper
            .expect_block()
            .withf(move |e, id, b| {
                e.lock().is_none()
                    && *id == network_identifier()
                    && *b
                        == PartialBlockIdentifier {
                            index: Some(index),
                            ..Default::default()
                        }
            })
            .return_const(Ok(b.clone()))
            .once();

        let tmp_b = b.clone().unwrap();
        syncer
            .handler
            .expect_block_seen()
            .withf(move |e, b| e.lock().is_none() && *b == tmp_b)
            .return_const(Ok(()))
            .once();
        let tmp_b = b.clone().unwrap();
        syncer
            .handler
            .expect_block_added()
            .withf(move |e, b| e.lock().is_none() && *b == Some(&tmp_b))
            .return_const(Ok(()))
            .once();
    }

    // Create reorg
    let new_blocks = create_blocks(790, 1200, "other");
    let block = new_blocks[11].clone().unwrap();
    let index = block.block_identifier.index.clone();
    syncer
        .helper
        .expect_block()
        .withf(move |e, id, b| {
            e.lock().is_none()
                && *id == network_identifier()
                && *b
                    == PartialBlockIdentifier {
                        index: Some(index),
                        ..Default::default()
                    }
        })
        .return_const(Ok(Some(block)))
        .once(); // [801]

    // Set parent of reorg start to be last good block
    new_blocks
        .get_mut(0)
        .unwrap()
        .as_mut()
        .unwrap()
        .parent_block_identifier = blocks[789].unwrap().block_identifier;

    // Orphan last 10 blocks
    for i in 790..=800 {
        let this_block = &new_blocks[i].unwrap();
        let index = this_block.block_identifier.index;
        syncer
            .helper
            .expect_block()
            .withf(move |e, id, b| {
                e.lock().is_none()
                    && *id == network_identifier()
                    && *b
                        == PartialBlockIdentifier {
                            index: Some(index),
                            ..Default::default()
                        }
            })
            .return_const(Ok(Some(this_block.clone())))
            .once();

        todo!("removed")
    }

    todo!()
}

#[test]
fn test_sync_manual_reorg() {
    todo!()
}

#[test]
fn test_sync_dynamic() {
    todo!()
}

#[test]
fn test_sync_dynamic_overhead() {
    todo!()
}
