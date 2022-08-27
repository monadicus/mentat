#![allow(clippy::missing_docs_in_private_items)]

use std::{
    sync::Arc,
    thread::{sleep, spawn},
    time::Duration,
};

use mentat_types::{
    AccountIdentifier,
    Amount,
    Block,
    BlockIdentifier,
    Currency,
    NetworkIdentifier,
    NetworkStatusResponse,
    Operation,
    OperationIdentifier,
    PartialBlockIdentifier,
    Transaction,
    TransactionIdentifier,
};
use mockall::{mock, TimesRange};
use parking_lot::{Mutex, MutexGuard};

use crate::{
    errors::{SyncerError, SyncerResult},
    syncer::BlockResult,
    types::{Handler, Helper, Syncer, SyncerBuilder, DEFAULT_CONCURRENCY},
    utils::Context,
};

mock! {
    #[allow(clippy::missing_docs_in_private_items)]
    pub Handler {}

    impl Handler for Handler {
        fn block_seen(&self, context: &Context<SyncerError>, block: &Block) -> SyncerResult<()>;
        fn block_added<'a, Hand: 'static, Help: 'static>(
            &self,
            syncer: &Syncer<Hand, Help>,
            context: &Context<SyncerError>,
            block: Option<Block>,
        ) -> SyncerResult<()>;
        fn block_removed<'a>(
            &self,
            context: &Context<SyncerError>,
            block: Option<&'a BlockIdentifier>,
        ) -> SyncerResult<()>;
    }
}

mock! {
    #[allow(clippy::missing_docs_in_private_items)]
    pub Helper {}

    impl Helper for Helper {
        fn network_status(
            &self,
            context: &Context<SyncerError>,
            network_identifier: &NetworkIdentifier,
        ) -> SyncerResult<NetworkStatusResponse>;

        fn block<Hand: 'static, Help: 'static>(
            &self,
            syncer: &Syncer<Hand, Help>,
            context: &Context<SyncerError>,
            network_identifier: &NetworkIdentifier,
            partial_block_identifier: &PartialBlockIdentifier,
        ) -> SyncerResult<Option<Block>>;
    }
}

/// MockHandler needs to be in arc so it can be cloned and expectations can be
/// shared across threads
#[derive(Clone)]
pub struct ArcMockHandler(Arc<Mutex<MockHandler>>);
impl ArcMockHandler {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(MockHandler::new())))
    }

    fn lock(&self) -> MutexGuard<MockHandler> {
        self.0.lock()
    }
}
impl Handler for ArcMockHandler {
    fn block_seen(&self, context: &Context<SyncerError>, block: &Block) -> SyncerResult<()> {
        self.0.lock().block_seen(context, block)
    }

    fn block_added<Hand: 'static, Help: 'static>(
        &self,
        syncer: &Syncer<Hand, Help>,
        context: &Context<SyncerError>,
        block: Option<Block>,
    ) -> SyncerResult<()> {
        self.0.lock().block_added(syncer, context, block)
    }

    fn block_removed<'a>(
        &self,
        context: &Context<SyncerError>,
        block: Option<&'a BlockIdentifier>,
    ) -> SyncerResult<()> {
        self.0.lock().block_removed(context, block)
    }
}

/// MockHelper needs to be in arc so it can be cloned and expectations can be
/// shared across threads
#[derive(Clone)]
pub struct ArcMockHelper(Arc<Mutex<MockHelper>>);
impl ArcMockHelper {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(MockHelper::new())))
    }

    fn lock(&self) -> MutexGuard<MockHelper> {
        self.0.lock()
    }
}
impl Helper for ArcMockHelper {
    fn network_status(
        &self,
        context: &Context<SyncerError>,
        network_identifier: &NetworkIdentifier,
    ) -> SyncerResult<NetworkStatusResponse> {
        self.0.lock().network_status(context, network_identifier)
    }

    fn block<Hand: 'static, Help: 'static>(
        &self,
        syncer: &Syncer<Hand, Help>,
        context: &Context<SyncerError>,
        network_identifier: &NetworkIdentifier,
        partial_block_identifier: &PartialBlockIdentifier,
    ) -> SyncerResult<Option<Block>> {
        self.0.lock().block(
            syncer,
            context,
            network_identifier,
            partial_block_identifier,
        )
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

fn block_sequence_idx(id: usize) -> Option<Block> {
    Some(block_sequence()[id].clone())
}

fn expect_block(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    enforce_buf: bool,
    index: Option<i64>,
    ret: SyncerResult<Option<Block>>,
    times: impl Into<TimesRange>,
) {
    let mut helper = syncer.helper.lock();
    helper
        .expect_block()
        .withf(move |_: &Syncer<ArcMockHandler, ArcMockHelper>, e, id, b| {
            !(e.done() && enforce_buf)
                && *id == network_identifier()
                && *b
                    == PartialBlockIdentifier {
                        index,
                        ..Default::default()
                    }
        })
        .return_const(ret)
        .times(times.into());
}

fn custom_expect_block<F>(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    index: Option<i64>,
    times: impl Into<TimesRange>,
    ret: F,
) where
    F: Fn(
            &Syncer<ArcMockHandler, ArcMockHelper>,
            &Context<SyncerError>,
            &NetworkIdentifier,
            &PartialBlockIdentifier,
        ) -> SyncerResult<Option<Block>>
        + Send
        + Sync
        + 'static,
{
    let mut helper = syncer.helper.lock();
    helper
        .expect_block()
        .withf(move |_: &Syncer<ArcMockHandler, ArcMockHelper>, e, id, b| {
            !e.done()
                && *id == network_identifier()
                && *b
                    == PartialBlockIdentifier {
                        index,
                        ..Default::default()
                    }
        })
        .returning(ret)
        .times(times.into());
}

fn expect_block_added(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    enforce_buf: bool,
    block: Option<Block>,
    times: impl Into<TimesRange>,
) {
    let mut handler = syncer.handler.lock();
    handler
        .expect_block_added()
        .withf(move |_: &Syncer<ArcMockHandler, ArcMockHelper>, e, g| {
            !(e.done() && enforce_buf) && *g == block
        })
        .return_const(Ok(()))
        .times(times.into());
}

fn custom_expect_block_added<F>(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    block: Option<Block>,
    times: impl Into<TimesRange>,
    ret: F,
) where
    F: Fn(
            &Syncer<ArcMockHandler, ArcMockHelper>,
            &Context<SyncerError>,
            Option<Block>,
        ) -> SyncerResult<()>
        + Send
        + Sync
        + 'static,
{
    let mut handler = syncer.handler.lock();
    handler
        .expect_block_added()
        .withf(move |_, e, g| !e.done() && *g == block)
        .returning(ret)
        .times(times.into());
}

fn expect_block_removed(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    id: Option<BlockIdentifier>,
    times: impl Into<TimesRange>,
) {
    let mut handler = syncer.handler.lock();
    handler
        .expect_block_removed()
        .withf(move |e, b| !e.done() && *b == id.as_ref())
        .return_const(Ok(()))
        .times(times.into());
}

fn expect_block_seen(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    enforce_buf: bool,
    block: Block,
    times: impl Into<TimesRange>,
) {
    let mut handler = syncer.handler.lock();
    handler
        .expect_block_seen()
        .withf(move |e, b| !(e.done() && enforce_buf) && *b == block)
        .return_const(Ok(()))
        .times(times.into());
}

fn expect_network_status(
    syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
    idx: i64,
    times: impl Into<TimesRange>,
) {
    let mut helper = syncer.helper.lock();
    helper
        .expect_network_status()
        .withf(|e, id| !e.done() && *id == network_identifier())
        .return_const(Ok(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: idx,
                hash: format!("block {idx}"),
            },
            genesis_block_identifier: BlockIdentifier {
                index: 0,
                hash: "block 0".into(),
            },
            ..Default::default()
        }))
        .times(times.into());
}

fn syncer() -> SyncerBuilder<ArcMockHandler, ArcMockHelper> {
    Syncer::builder(
        network_identifier(),
        ArcMockHelper::new(),
        ArcMockHandler::new(),
    )
}

fn buf() -> Context<SyncerError> {
    Context::new(Some(Duration::from_secs(180)))
}

#[test]
fn test_process_block() {
    let context = buf();
    let mut syncer = Syncer::builder(
        network_identifier(),
        ArcMockHelper::new(),
        ArcMockHandler::new(),
    )
    .build();
    syncer.genesis_block = block_sequence_idx(0).map(|b| b.block_identifier);
    assert!(syncer.past_blocks.is_empty());

    let assert_syncer = |syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
                         next_index: i64,
                         past_blocks: &[usize]| {
        assert_eq!(syncer.next_index, next_index);
        assert_eq!(
            &syncer.past_blocks,
            &past_blocks
                .iter()
                .map(|b| block_sequence_idx(*b).unwrap().block_identifier)
                .collect::<Vec<_>>()
        );
        syncer.handler.lock().checkpoint();
    };

    let process_block = |syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>,
                         block: Option<Block>| {
        syncer.process_block(
            &context,
            Some(BlockResult {
                block,
                ..Default::default()
            }),
        )
    };

    {
        print!("No block exists: ");
        expect_block_added(&mut syncer, true, block_sequence_idx(0), 1);
        process_block(&mut syncer, block_sequence_idx(0)).unwrap();
        assert_syncer(&mut syncer, 1, &[0]);
        println!("ok!");
    }

    {
        print!("Orphan genesis: ");
        let err = process_block(&mut syncer, Some(orphan_genesis())).unwrap_err();
        assert!(
            err.to_string()
                .contains(&SyncerError::CannotRemoveGenesisBlock.to_string())
        );
        assert_syncer(&mut syncer, 1, &[0]);
        println!("ok!");
    }

    {
        print!("Block exists, no reorg: ");
        expect_block_added(&mut syncer, true, block_sequence_idx(1), 1);
        process_block(&mut syncer, block_sequence_idx(1)).unwrap();
        assert_syncer(&mut syncer, 2, &[0, 1]);
        println!("ok!");
    }

    {
        print!("Orphan block: ");
        expect_block_removed(
            &mut syncer,
            block_sequence_idx(1).map(|b| b.block_identifier),
            1,
        );
        process_block(&mut syncer, block_sequence_idx(2)).unwrap();
        assert_syncer(&mut syncer, 1, &[0]);

        expect_block_added(&mut syncer, true, block_sequence_idx(3), 1);
        process_block(&mut syncer, block_sequence_idx(3)).unwrap();
        assert_syncer(&mut syncer, 2, &[0, 3]);

        expect_block_added(&mut syncer, true, block_sequence_idx(2), 1);
        process_block(&mut syncer, block_sequence_idx(2)).unwrap();
        assert_syncer(&mut syncer, 3, &[0, 3, 2]);
        println!("ok!");
    }

    {
        print!("Out of order block: ");
        let err = process_block(&mut syncer, block_sequence_idx(5)).unwrap_err();
        assert!(
            err.to_string()
                .contains("expected block index 3, but got 5")
        );
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
        let err = syncer.process_block(&context, None).unwrap_err();
        assert_eq!(err, SyncerError::BlockResultNil);
        println!("ok!");
    }

    {
        print!("Process orphan head block result: ");
        expect_block_removed(
            &mut syncer,
            block_sequence_idx(2).map(|b| b.block_identifier),
            1,
        );
        syncer
            .process_block(
                &context,
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
    let mut syncer = syncer().max_concurrency(3).build();

    // Tip should be nil before we start syncing
    assert!(syncer.tip.is_none());

    // Force syncer to only get part of the way through the full range
    expect_network_status(&mut syncer, 200, 2);
    expect_network_status(&mut syncer, 1300, 2);

    let mut blocks = create_blocks(0, 1200, "");
    // Create a block gap
    blocks[100] = None;
    blocks[101].as_mut().unwrap().parent_block_identifier =
        blocks[99].as_ref().unwrap().block_identifier.clone();

    for (i, b) in blocks.into_iter().enumerate() {
        expect_block(&mut syncer, true, Some(i as i64), Ok(b.clone()), 1);

        if let Some(b) = b {
            expect_block_seen(&mut syncer, true, b.clone(), 1);
            custom_expect_block_added(&mut syncer, Some(b.clone()), 1, move |s, _, _| {
                let id = if i > 200 {
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

                assert_eq!(Some(&id), s.tip());
                Ok(())
            });
        }
    }

    syncer.sync(&buf(), -1, 1200).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.lock().checkpoint();
    syncer.handler.lock().checkpoint();
}

// TODO: infinite lock
#[test]
fn test_sync_specific_start() {
    let mut syncer = syncer().build();

    expect_network_status(&mut syncer, 1300, ..);

    for b in create_blocks(100, 1200, "") {
        expect_block(
            &mut syncer,
            true,
            b.as_ref().map(|b| b.block_identifier.index),
            Ok(b.clone()),
            1,
        );
        expect_block_seen(&mut syncer, true, b.clone().unwrap(), 1);
        custom_expect_block_added(&mut syncer, b.clone(), 1, |s, _, b| {
            if b.unwrap().block_identifier.index == 800 {
                assert!(*s.concurrency.lock() > DEFAULT_CONCURRENCY)
            }
            Ok(())
        });
    }

    syncer.sync(&buf(), 100, 1200).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.lock().checkpoint();
    syncer.handler.lock().checkpoint();
}

#[test]
fn test_sync_cancel() {
    let mut syncer = syncer().build();

    expect_network_status(&mut syncer, 900, ..=2);
    expect_network_status(&mut syncer, 1300, ..=2);

    for b in create_blocks(0, 1200, "") {
        expect_block(
            &mut syncer,
            false,
            b.as_ref().map(|b| b.block_identifier.index),
            Ok(b.clone()),
            ..=1,
        );
        expect_block_seen(&mut syncer, false, b.clone().unwrap(), ..=1);
        expect_block_added(&mut syncer, false, b.clone(), ..=1);
    }

    let context = buf();
    let tmp_ctx = context.clone();
    let handle = spawn(move || {
        sleep(Duration::from_secs(1));
        tmp_ctx.cancel()
    });
    let err = syncer.sync(&context, -1, 1200).unwrap_err();
    assert_eq!(err, SyncerError::Canceled);
    assert_eq!(0, *syncer.concurrency.lock());
    handle.join().unwrap();
}

#[test]
fn test_sync_reorg() {
    let mut syncer = syncer().build();

    expect_network_status(&mut syncer, 1300, ..);

    let blocks = create_blocks(0, 800, "");
    // [0, 800]
    for b in &blocks {
        expect_block(
            &mut syncer,
            true,
            b.as_ref().map(|b| b.block_identifier.index),
            Ok(b.clone()),
            1,
        );

        expect_block_seen(&mut syncer, true, b.clone().unwrap(), 1);
        expect_block_added(&mut syncer, true, b.clone(), 1);
    }

    // Create reorg
    let mut new_blocks = create_blocks(790, 1200, "other");
    let block = new_blocks[11].clone();

    // [801]
    expect_block(
        &mut syncer,
        true,
        block.as_ref().map(|b| b.block_identifier.index),
        Ok(block),
        1,
    );

    // Set parent of reorg start to be last good block
    new_blocks
        .get_mut(0)
        .unwrap()
        .as_mut()
        .unwrap()
        .parent_block_identifier = blocks[789].as_ref().unwrap().block_identifier.clone();

    // Orphan last 10 blocks
    for i in 790..=800 {
        let this_block = new_blocks[i - 790].clone();
        expect_block(
            &mut syncer,
            true,
            this_block.as_ref().map(|b| b.block_identifier.index),
            Ok(this_block),
            1,
        );
        expect_block_removed(
            &mut syncer,
            blocks[i].as_ref().map(|b| b.block_identifier.clone()),
            1,
        );
    }

    let block = new_blocks[0].clone();
    // only fetch these blocks once
    expect_block_seen(&mut syncer, true, block.clone().unwrap(), 1);
    expect_block_added(&mut syncer, true, block, 1);

    // New blocks added
    // [790, 1200]
    for b in &new_blocks[1..] {
        expect_block(
            &mut syncer,
            true,
            b.as_ref().map(|b| b.block_identifier.index),
            Ok(b.clone()),
            1,
        );

        let b = b.as_ref().unwrap();
        let seen_times = if b.block_identifier.index > 801 { 1 } else { 2 };
        expect_block_seen(&mut syncer, true, b.clone(), seen_times);
        custom_expect_block_added(&mut syncer, Some(b.clone()), 1, |s, _, b| {
            if b.unwrap().block_identifier.index == 800 {
                assert!(*s.concurrency.lock() > DEFAULT_CONCURRENCY)
            }
            Ok(())
        });
    }

    // Expected Calls to Block
    // [0, 789] = 1
    // [790] = 2
    // [791, 800] = 3
    // [801] = 2
    // [802,1200] = 1

    syncer.sync(&buf(), -1, 1200).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.lock().checkpoint();
    syncer.handler.lock().checkpoint();
}

// TODO: infinite hang
#[test]
fn test_sync_manual_reorg() {
    let mut syncer = syncer().build();

    expect_network_status(&mut syncer, 1300, ..);

    let blocks = create_blocks(0, 800, "");
    // [0, 800]
    for b in &blocks {
        expect_block(
            &mut syncer,
            true,
            b.as_ref().map(|b| b.block_identifier.index),
            Ok(b.clone()),
            1,
        );
        expect_block_seen(&mut syncer, true, b.clone().unwrap(), 1);
        expect_block_added(&mut syncer, true, b.clone(), 1);
    }

    // Create reorg
    // [801]
    expect_block(
        &mut syncer,
        true,
        Some(801),
        Err(SyncerError::OrphanHead),
        1,
    );
    expect_block_removed(
        &mut syncer,
        blocks
            .last()
            .unwrap()
            .as_ref()
            .map(|b| b.block_identifier.clone()),
        1,
    );

    let new_blocks = create_blocks(800, 1200, "");
    // [800, 1200]
    for b in &new_blocks {
        expect_block(
            &mut syncer,
            true,
            b.as_ref().map(|b| b.block_identifier.index),
            Ok(b.clone()),
            1,
        );
        expect_block_seen(&mut syncer, true, b.clone().unwrap(), 1);
        custom_expect_block_added(&mut syncer, b.clone(), 1, |s, _, b| {
            if b.unwrap().block_identifier.index == 800 {
                assert!(*s.concurrency.lock() > DEFAULT_CONCURRENCY)
            }
            Ok(())
        });
    }

    // Expected Calls to Block
    // [0, 799] = 1
    // [800, 801] = 2
    // [802,1200] = 1

    syncer.sync(&buf(), -1, 1200).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.lock().checkpoint();
    syncer.handler.lock().checkpoint();
}

fn sync_dynamic(syncer: &mut Syncer<ArcMockHandler, ArcMockHelper>) {
    // Force syncer to only get part of the way through the full range
    expect_network_status(syncer, 1, 2);
    expect_network_status(syncer, 1300, 2);

    let mut blocks = create_blocks(0, 200, "");
    // Load blocks with a ton of transactions
    for block in &mut blocks {
        let mut block = block.as_mut().unwrap();
        block.transactions = (0..10000)
            .into_iter()
            .map(|i| Transaction {
                transaction_identifier: TransactionIdentifier {
                    hash: format!("block {} tx {}", block.block_identifier.index, i),
                },
                ..Default::default()
            })
            .collect();
    }

    // Create a block gap
    blocks[100] = None;
    blocks[101].as_mut().unwrap().parent_block_identifier =
        blocks[99].as_ref().unwrap().block_identifier.clone();

    for (i, b) in blocks.into_iter().enumerate() {
        let tmp_b = b.clone();
        custom_expect_block(syncer, Some(i as i64), 1, move |s, _, _, id| {
            if id.index == Some(100) {
                assert_eq!(*s.concurrency.lock(), 1)
            }
            Ok(tmp_b.clone())
        });

        if let Some(b) = b {
            expect_block_seen(syncer, true, b.clone(), 1);
            expect_block_added(syncer, true, Some(b), 1);
        }
    }

    syncer.sync(&buf(), -1, 200).unwrap();
    assert_eq!(0, *syncer.concurrency.lock());
    syncer.helper.lock().checkpoint();
    syncer.handler.lock().checkpoint();
}

#[test]
fn test_sync_dynamic() {
    let mut syncer = syncer()
        // 1 MB
        .cache_size(1 << 20)
        .build();
    sync_dynamic(&mut syncer);
}

#[test]
fn test_sync_dynamic_overhead() {
    let mut syncer = syncer()
        // 1 MB
        .cache_size(1 << 20)
        // greatly increase synthetic size
        .size_multiplier(100000.0)
        .build();
    sync_dynamic(&mut syncer);
}
