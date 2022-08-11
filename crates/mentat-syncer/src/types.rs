//! TODO

use crate::types::BlockIdentifier;
use crate::{
    errors::SyncerResult,
    types::{Block, NetworkIdentifier, NetworkStatusResponse, PartialBlockIdentifier},
};
use mentat_types::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// DEFAULT_PAST_BLOCK_LIMIT is the maximum number of previously
/// processed block headers we keep in the syncer to handle
/// reorgs correctly. If there is a reorg greater than
/// DEFAULT_PAST_BLOCK_LIMIT, it will not be handled correctly.
pub const DEFAULT_PAST_BLOCK_LIMIT: u64 = 100;

/// DEFAULT_CONCURRENCY is the default number of
/// blocks the syncer will try to get concurrently.
pub const DEFAULT_CONCURRENCY: u64 = 4;

/// DEFAULT_CACHE_SIZE is the default size of the preprocess
/// cache for the syncer.
pub const DEFAULT_CACHE_SIZE: u64 = 2000 << 20; // 2 GB

/// LARGE_CACHE_SIZE will aim to use 5 GB of memory.
pub const LARGE_CACHE_SIZE: u64 = 5000 << 20; // 5 GB

/// SMALL_CACHE_SIZE will aim to use 500 MB of memory.
pub const SMALL_CACHE_SIZE: u64 = 500 << 20; // 500 MB

/// TINY_CACHE_SIZE will aim to use 200 MB of memory.
pub const TINY_CACHE_SIZE: u64 = 200 << 20; // 200 MB

/// DEFAULT_MAX_CONCURRENCY is the maximum concurrency we will
/// attempt to sync with.
pub const DEFAULT_MAX_CONCURRENCY: u64 = 256;

/// MIN_CONCURRENCY is the minimum concurrency we will
/// attempt to sync with.
pub const MIN_CONCURRENCY: u64 = 1;

/// DEFAULT_TRAILING_WINDOW is the size of the trailing window
/// of block sizes to keep when adjusting concurrency.
pub const DEFAULT_TRAILING_WINDOW: u64 = 1000;

/// DEFAULT_ADJUSTMENT_WINDOW is how frequently we will
/// consider increasing our concurrency.
pub const DEFAULT_ADJUSTMENT_WINDOW: u64 = 5;

/// DEFAULT_SIZE_MULTIPLIER is used to pad our average size adjustment.
/// This can be used to account for the overhead associated with processing
/// a particular block (i.e. balance adjustments, coins created, etc).
pub const DEFAULT_SIZE_MULTIPLIER: f64 = 10.0;

/// DEFAULT_SYNC_SLEEP is the amount of time to sleep
/// when we are at tip but want to keep syncing.
pub const DEFAULT_SYNC_SLEEP: Duration = Duration::from_secs(2);

/// DEFAULT_FETCH_SLEEP is the amount of time to sleep
/// when we are loading more blocks to fetch but we
/// already have a backlog >= to concurrency.
pub const DEFAULT_FETCH_SLEEP: Duration = Duration::from_millis(500);

/// Handler is called at various times during the sync cycle
/// to handle different events. It is common to write logs or
/// perform reconciliation in the sync processor.
pub trait Handler {
    /// BlockSeen is invoked AT LEAST ONCE
    /// by the syncer prior to calling BlockAdded
    /// with the same arguments. This allows for
    /// storing block data before it is sequenced.
    fn block_seen(ctx: (), block: &Block) -> SyncerResult<()>;
    #[allow(clippy::missing_docs_in_private_items)]
    fn block_added(ctx: (), block: &Block) -> SyncerResult<()>;
    #[allow(clippy::missing_docs_in_private_items)]
    fn block_removed(ctx: (), block: &BlockIdentifier) -> SyncerResult<()>;
}

/// Helper is called at various times during the sync cycle
/// to get information about a blockchain network. It is
/// common to implement this helper using the Fetcher package.
#[allow(clippy::missing_docs_in_private_items)]
pub trait Helper {
    fn network_status(
        ctx: (),
        network_identifier: &NetworkIdentifier,
    ) -> SyncerResult<NetworkStatusResponse>;

    fn block(
        ctx: (),
        network_identifier: &NetworkIdentifier,
        partial_block_identifier: &PartialBlockIdentifier,
    ) -> SyncerResult<Block>;
}

/// Syncer coordinates blockchain syncing without relying on
/// a storage interface. Instead, it calls a provided Handler
/// whenever a block is added or removed. This provides the client
/// the opportunity to define the logic used to handle each new block.
/// In the rosetta-cli, we handle reconciliation, state storage, and
/// logging in the handler.
#[allow(clippy::missing_docs_in_private_items)]
pub struct Syncer<'net_id, 'block_id, Hand, Help> {
    pub network: &'net_id NetworkIdentifier,
    pub helper: Help,
    pub handler: Hand,
    // TODO context.CancelFunc: A CancelFunc tells an operation to abandon its work.
    //  A CancelFunc does not wait for the work to stop. A CancelFunc may be called by
    //  multiple goroutines simultaneously. After the first call, subsequent calls to
    //  a CancelFunc do nothing.
    pub cancel: (),

    /// Used to keep track of sync state
    pub genesis_block: BlockIdentifier,
    pub tip: BlockIdentifier,
    pub next_index: i64,

    /// To ensure reorgs are handled correctly, the syncer must be able
    /// to observe blocks it has previously processed. Without this, the
    /// syncer may process an index that is not connected to previously added
    /// blocks (ParentBlockIdentifier != lastProcessedBlock.BlockIdentifier).
    ///
    /// If a blockchain does not have reorgs, it is not necessary to populate
    /// the blockCache on creation.
    pub past_blocks: Vec<&'block_id BlockIdentifier>,
    pub past_block_limit: i64,

    /// Automatically manage concurrency based on the
    /// provided max cache size. The algorithm used here
    /// is a slow rise (to increase concurrency) and fast
    /// fall (if we breach our max cache size).
    pub cache_size: i64,
    pub size_multiplier: f64,
    pub max_concurrency: i64,
    pub concurrency: Arc<Mutex<i64>>,
    pub goal_concurrency: i64,
    pub recent_block_sizes: Vec<i64>,
    pub last_adjustment: i64,
    pub adjustment_window: i64,

    /// doneLoading is used to coordinate adding goroutines
    /// when close to the end of syncing a range.
    pub done_loading: Arc<Mutex<bool>>,
}

impl<'net_id, 'block_id, Hand, Help> Syncer<'net_id, 'block_id, Hand, Help> {
    /// TODO doc
    pub fn builder(
        network: &'net_id NetworkIdentifier,
        helper: Help,
        handler: Hand,
        cancel: (), // context::CancelFunc,
    ) -> SyncerBuilder<'net_id, 'block_id, Hand, Help> {
        SyncerBuilder::new(network, helper, handler, cancel)
    }
}

/// A builder new Syncer. If `past_blocks` is left nil, it will
/// be set to an empty slice.
#[allow(clippy::missing_docs_in_private_items)]
pub struct SyncerBuilder<'net_id, 'block_id, Hand, Help> {
    network: &'net_id NetworkIdentifier,
    helper: Help,
    handler: Hand,
    cancel: (),
    past_blocks: Option<Vec<&'block_id BlockIdentifier>>,
    past_block_limit: Option<i64>,
    cache_size: Option<i64>,
    size_multiplier: Option<f64>,
    max_concurrency: Option<i64>,
    adjustment_window: Option<i64>,
}

#[allow(clippy::missing_docs_in_private_items)]
impl<'net_id, 'block_id, Hand, Help> SyncerBuilder<'net_id, 'block_id, Hand, Help> {
    pub fn new(
        network: &'net_id NetworkIdentifier,
        helper: Help,
        handler: Hand,
        cancel: (), // context::CancelFunc,
    ) -> Self {
        Self {
            network,
            helper,
            handler,
            past_blocks: None,
            past_block_limit: None,
            cache_size: None,
            size_multiplier: None,
            cancel,
            max_concurrency: None,
            adjustment_window: None,
        }
    }

    pub fn cache_size(mut self, v: i64) -> Self {
        self.cache_size = Some(v);
        self
    }

    pub fn size_multiplier(mut self, v: f64) -> Self {
        self.size_multiplier = Some(v);
        self
    }

    pub fn past_blocks(mut self, v: Vec<&'block_id BlockIdentifier>) -> Self {
        self.past_blocks = Some(v);
        self
    }

    pub fn past_block_limit(mut self, v: i64) -> Self {
        self.past_block_limit = Some(v);
        self
    }

    pub fn max_concurrency(mut self, v: i64) -> Self {
        self.max_concurrency = Some(v);
        self
    }

    pub fn adjustment_window(mut self, v: i64) -> Self {
        self.adjustment_window = Some(v);
        self
    }

    pub fn build(self) -> Syncer<'net_id, 'block_id, Hand, Help> {
        Syncer {
            network: self.network,
            helper: self.helper,
            handler: self.handler,
            cancel: self.cancel,
            genesis_block: Default::default(),
            tip: Default::default(),
            next_index: Default::default(),
            past_blocks: self.past_blocks.unwrap_or_default(),
            past_block_limit: self.past_block_limit.unwrap_or_default(),
            cache_size: self.cache_size.unwrap_or_default(),
            size_multiplier: self.size_multiplier.unwrap_or_default(),
            max_concurrency: self.max_concurrency.unwrap_or_default(),
            concurrency: Default::default(),
            goal_concurrency: Default::default(),
            recent_block_sizes: Default::default(),
            last_adjustment: Default::default(),
            adjustment_window: self.adjustment_window.unwrap_or_default(),
            done_loading: Default::default(),
        }
    }
}
