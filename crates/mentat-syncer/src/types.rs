//! types used to implement mentat-syncer

use super::*;

/// DEFAULT_PAST_BLOCK_LIMIT is the maximum number of previously
/// processed block headers we keep in the syncer to handle
/// reorgs correctly. If there is a reorg greater than
/// DEFAULT_PAST_BLOCK_LIMIT, it will not be handled correctly.
pub const DEFAULT_PAST_BLOCK_LIMIT: usize = 100;

/// DEFAULT_CONCURRENCY is the default number of
/// blocks the syncer will try to get concurrently.
pub const DEFAULT_CONCURRENCY: usize = 4;

/// DEFAULT_CACHE_SIZE is the default size of the preprocess
/// cache for the syncer.
pub const DEFAULT_CACHE_SIZE: usize = 2000 << 20; // 2 GB

/// LARGE_CACHE_SIZE will aim to use 5 GB of memory.
pub const LARGE_CACHE_SIZE: usize = 5000 << 20; // 5 GB

/// SMALL_CACHE_SIZE will aim to use 500 MB of memory.
pub const SMALL_CACHE_SIZE: usize = 500 << 20; // 500 MB

/// TINY_CACHE_SIZE will aim to use 200 MB of memory.
pub const TINY_CACHE_SIZE: usize = 200 << 20; // 200 MB

/// DEFAULT_MAX_CONCURRENCY is the maximum concurrency we will
/// attempt to sync with.
pub const DEFAULT_MAX_CONCURRENCY: usize = 256;

/// MIN_CONCURRENCY is the minimum concurrency we will
/// attempt to sync with.
pub const MIN_CONCURRENCY: usize = 1;

/// DEFAULT_TRAILING_WINDOW is the size of the trailing window
/// of block sizes to keep when adjusting concurrency.
pub const DEFAULT_TRAILING_WINDOW: usize = 1000;

/// DEFAULT_ADJUSTMENT_WINDOW is how frequently we will
/// consider increasing our concurrency.
pub const DEFAULT_ADJUSTMENT_WINDOW: usize = 5;

/// DEFAULT_SIZE_MULTIPLIER is used to pad our average size adjustment.
/// This can be used to account for the overhead associated with processing
/// a particular block (i.e. balance adjustments, coins created, etc).
pub const DEFAULT_SIZE_MULTIPLIER: f64 = 10.0;

/// DEFAULT_SYNC_SLEEP is the amount of time to sleep
/// when we are at tip but want to keep syncing.
pub const DEFAULT_SYNC_SLEEP: Duration = Duration::from_secs(2);

/// Handler is called at various times during the sync cycle
/// to handle different events. It is common to write logs or
/// perform reconciliation in the sync processor.
#[allow(clippy::missing_docs_in_private_items)]
pub trait Handler {
    /// BlockSeen is invoked AT LEAST ONCE
    /// by the syncer prior to calling BlockAdded
    /// with the same arguments. This allows for
    /// storing block data before it is sequenced.
    fn block_seen(&self, context: &Context<SyncerError>, block: &Block) -> SyncerResult<()>;
    #[cfg(not(test))]
    fn block_added(
        &self,
        context: &Context<SyncerError>,
        block: Option<&Block>,
    ) -> SyncerResult<()>;
    // mock structures inside syncer_tests require access to syncer during this
    // method for certain checks
    #[cfg(test)]
    fn block_added<Hand: 'static, Help: 'static>(
        &self,
        syncer: &Syncer<Hand, Help>,
        context: &Context<SyncerError>,
        block: Option<Block>,
    ) -> SyncerResult<()>;
    fn block_removed(
        &self,
        context: &Context<SyncerError>,
        block: Option<&BlockIdentifier>,
    ) -> SyncerResult<()>;
}

/// Helper is called at various times during the sync cycle
/// to get information about a blockchain network. It is
/// common to implement this helper using the Fetcher package.
#[allow(clippy::missing_docs_in_private_items)]
pub trait Helper {
    fn network_status(
        &self,
        context: &Context<SyncerError>,
        network_identifier: &NetworkIdentifier,
    ) -> SyncerResult<NetworkStatusResponse>;

    // TODO this should probably return a reference?
    #[cfg(not(test))]
    fn block(
        &self,
        context: &Context<SyncerError>,
        network_identifier: &NetworkIdentifier,
        partial_block_identifier: &PartialBlockIdentifier,
    ) -> SyncerResult<Option<Block>>;

    // mock structures inside syncer_tests require access to syncer during this
    // method for certain checks
    #[cfg(test)]
    fn block<Hand: 'static, Help: 'static>(
        &self,
        syncer: &Syncer<Hand, Help>,
        context: &Context<SyncerError>,
        network_identifier: &NetworkIdentifier,
        partial_block_identifier: &PartialBlockIdentifier,
    ) -> SyncerResult<Option<Block>>;
}

/// Syncer coordinates blockchain syncing without relying on
/// a storage interface. Instead, it calls a provided Handler
/// whenever a block is added or removed. This provides the client
/// the opportunity to define the logic used to handle each new block.
/// In the rosetta-cli, we handle reconciliation, state storage, and
/// logging in the handler.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone)]
pub struct Syncer<Handler, Helper> {
    pub network: NetworkIdentifier,
    pub helper: Helper,
    pub handler: Handler,

    /// Used to keep track of sync state
    pub genesis_block: Option<BlockIdentifier>,
    pub tip: Option<BlockIdentifier>,
    pub next_index: usize,

    /// To ensure reorgs are handled correctly, the syncer must be able
    /// to observe blocks it has previously processed. Without this, the
    /// syncer may process an index that is not connected to previously added
    /// blocks (ParentBlockIdentifier != lastProcessedBlock.BlockIdentifier).
    ///
    /// If a blockchain does not have reorgs, it is not necessary to populate
    /// the blockCache on creation.
    pub past_blocks: VecDeque<BlockIdentifier>,
    pub past_block_limit: usize,

    /// Automatically manage concurrency based on the
    /// provided max cache size. The algorithm used here
    /// is a slow rise (to increase concurrency) and fast
    /// fall (if we breach our max cache size).
    pub cache_size: usize,
    pub size_multiplier: f64,
    pub max_concurrency: usize,
    pub concurrency: Arc<Mutex<usize>>,
    pub goal_concurrency: Arc<Mutex<usize>>,
    pub recent_block_sizes: VecDeque<usize>,
    pub last_adjustment: usize,
    pub adjustment_window: usize,
}

impl<Handler, Helper> Syncer<Handler, Helper> {
    /// TODO doc
    pub fn builder(
        network: NetworkIdentifier,
        helper: Helper,
        handler: Handler,
    ) -> SyncerBuilder<Handler, Helper> {
        SyncerBuilder::<Handler, Helper>::new(network, helper, handler)
    }
}

/// A builder new Syncer. If `past_blocks` is left nil, it will
/// be set to an empty slice.
#[allow(clippy::missing_docs_in_private_items)]
pub struct SyncerBuilder<Handler, Helper> {
    network: NetworkIdentifier,
    helper: Helper,
    handler: Handler,
    past_blocks: Option<Vec<BlockIdentifier>>,
    past_block_limit: Option<usize>,
    cache_size: Option<usize>,
    size_multiplier: Option<f64>,
    max_concurrency: Option<usize>,
    adjustment_window: Option<usize>,
}

#[allow(clippy::missing_docs_in_private_items)]
impl<Handler, Helper> SyncerBuilder<Handler, Helper> {
    pub fn new(network: NetworkIdentifier, helper: Helper, handler: Handler) -> Self {
        Self {
            network,
            helper,
            handler,
            past_blocks: None,
            past_block_limit: None,
            cache_size: None,
            size_multiplier: None,
            max_concurrency: None,
            adjustment_window: None,
        }
    }

    pub fn cache_size(mut self, v: usize) -> Self {
        self.cache_size = Some(v);
        self
    }

    pub fn size_multiplier(mut self, v: f64) -> Self {
        self.size_multiplier = Some(v);
        self
    }

    pub fn past_blocks(mut self, v: Vec<BlockIdentifier>) -> Self {
        self.past_blocks = Some(v);
        self
    }

    pub fn past_block_limit(mut self, v: usize) -> Self {
        self.past_block_limit = Some(v);
        self
    }

    pub fn max_concurrency(mut self, v: usize) -> Self {
        self.max_concurrency = Some(v);
        self
    }

    pub fn adjustment_window(mut self, v: usize) -> Self {
        self.adjustment_window = Some(v);
        self
    }

    pub fn build(self) -> Syncer<Handler, Helper> {
        Syncer {
            network: self.network,
            helper: self.helper,
            handler: self.handler,
            genesis_block: Default::default(),
            tip: Default::default(),
            next_index: Default::default(),
            past_blocks: self.past_blocks.unwrap_or_default().into(),
            past_block_limit: self.past_block_limit.unwrap_or(DEFAULT_PAST_BLOCK_LIMIT),
            cache_size: self.cache_size.unwrap_or(DEFAULT_CACHE_SIZE),
            size_multiplier: self.size_multiplier.unwrap_or(DEFAULT_SIZE_MULTIPLIER),
            max_concurrency: self.max_concurrency.unwrap_or(DEFAULT_MAX_CONCURRENCY),
            concurrency: Arc::new(Mutex::new(DEFAULT_CONCURRENCY)),
            goal_concurrency: Default::default(),
            recent_block_sizes: Default::default(),
            last_adjustment: Default::default(),
            adjustment_window: self.adjustment_window.unwrap_or(DEFAULT_ADJUSTMENT_WINDOW),
        }
    }
}
