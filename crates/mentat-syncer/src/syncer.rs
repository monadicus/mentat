//! TODO

use std::{
    mem::size_of_val,
    sync::Arc,
    thread::{sleep, spawn},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use indexmap::IndexMap;
use mentat_types::{hash, Block, BlockIdentifier, EstimateSize, PartialBlockIdentifier};
use parking_lot::MutexGuard;

use crate::{
    errors::{SyncerError, SyncerResult},
    types::{
        Handler, Helper, Syncer, DEFAULT_CONCURRENCY, DEFAULT_FETCH_SLEEP, DEFAULT_SYNC_SLEEP,
        DEFAULT_TRAILING_WINDOW, MIN_CONCURRENCY,
    },
    utils::{Context, ThreadHandler},
};

/// blockResult is returned by calls
/// to fetch a particular index. We must
/// use a separate index field in case
/// the block is omitted and we can't
/// determine the index of the request.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Default, Clone, Debug)]
pub struct BlockResult {
    pub index: i64,
    pub block: Option<Block>,
    pub orphaned_head: bool,
}

impl EstimateSize for BlockResult {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + self
                .block
                .as_ref()
                .map(|b| b.estimated_size())
                .unwrap_or_default()
    }
}

impl<Hand, Help> Syncer<Hand, Help>
where
    Hand: 'static + Handler + Send + Sync + Clone,
    Help: 'static + Helper + Send + Sync + Clone,
{
    #[allow(clippy::missing_docs_in_private_items)]
    fn set_start(&mut self, context: &Context<SyncerError>, index: i64) -> SyncerResult<()> {
        let network_status = self
            .helper
            .network_status(context, &self.network)
            .map_err(|e| {
                format!(
                    "unable to get network status of {}: {e}",
                    self.network.network
                )
            })?;
        self.genesis_block = Some(network_status.genesis_block_identifier);
        if index != -1 {
            self.next_index = index
        } else {
            self.next_index = self.genesis_block.as_ref().unwrap().index
        }
        Ok(())
    }

    /// nextSyncableRange returns the next range of indexes to sync
    /// based on what the last processed block in storage is and
    /// the contents of the network status response.
    fn next_syncable_range(
        &mut self,
        context: &Context<SyncerError>,
        mut end_index: i64,
    ) -> SyncerResult<(i64, bool)> {
        if self.next_index == -1 {
            return Err(SyncerError::GetCurrentHeadBlockFailed);
        }

        // Always fetch network status to ensure endIndex is not past tip
        let network_status = self
            .helper
            .network_status(context, &self.network)
            .map_err(|e| {
                format!(
                    "unable to get network status of {}: {e}",
                    self.network.network
                )
            })?;

        // Update the syncer's known tip
        let current_block_identifier_index = network_status.current_block_identifier.index;
        self.tip = Some(network_status.current_block_identifier);

        if end_index == -1 || end_index > current_block_identifier_index {
            end_index = current_block_identifier_index
        }

        if self.next_index > end_index {
            Ok((-1, true))
        } else {
            Ok((end_index, false))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    fn attempt_orphan<'a>(
        &self,
        last_block: &'a BlockIdentifier,
    ) -> SyncerResult<(bool, Option<&'a BlockIdentifier>)> {
        if hash(self.genesis_block.as_ref()) == hash(Some(last_block)) {
            Err(SyncerError::CannotRemoveGenesisBlock)
        } else {
            Ok((true, Some(last_block)))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    fn check_remove<'a>(
        &'a self,
        br: &BlockResult,
    ) -> SyncerResult<(bool, Option<&'a BlockIdentifier>)> {
        if self.past_blocks.is_empty() {
            return Ok((false, None));
        }

        let last_block = self.past_blocks.back().unwrap();
        if br.orphaned_head {
            return self.attempt_orphan(last_block);
        }

        // Ensure processing correct index
        let block = br.block.as_ref().unwrap();
        if block.block_identifier.index != self.next_index {
            Err(format!(
                "expected block index {}, but got {}: {}",
                self.next_index,
                block.block_identifier.index,
                SyncerError::OutOfOrder,
            )
            .into())
        } else if hash(Some(&block.parent_block_identifier)) != hash(Some(last_block)) {
            // Check if block parent is head
            self.attempt_orphan(last_block)
        } else {
            Ok((false, Some(last_block)))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn process_block(
        &mut self,
        context: &Context<SyncerError>,
        br: Option<BlockResult>,
    ) -> SyncerResult<()> {
        let br = br.ok_or(SyncerError::BlockResultNil)?;
        if br.block.is_none() && !br.orphaned_head {
            // If the block is omitted, increase
            // index and return.
            self.next_index += 1;
        } else if let (true, last_block) = self.check_remove(&br).map_err(|e| {
            format!(
                "failed to check if the last block should be removed when processing block {}: {e}",
                br.index
            )
        })? {
            self.handler
                .block_removed(context, last_block)
                .map_err(|e| {
                    format!(
                        "failed to handle the event of block {} is removed: {e}",
                        last_block.unwrap().index,
                    )
                })?;
            self.next_index = last_block.unwrap().index;
            self.past_blocks.pop_back();
        } else {
            // mock structures inside syncer_tests require access to syncer during this method for certain checks
            #[cfg(test)]
            let r = self.handler.block_added(self, context, br.block.clone());
            #[cfg(not(test))]
            let r = self.handler.block_added(context, br.block.as_ref());
            let block = br.block.unwrap();
            r.map_err(|e| {
                format!(
                    "failed to handle the event of block {} is added: {e}",
                    block.block_identifier.index
                )
            })?;
            let idx = block.block_identifier.index;
            self.past_blocks.push_back(block.block_identifier);
            if self.past_blocks.len() > self.past_block_limit as usize {
                self.past_blocks.pop_front();
            }
            self.next_index = idx + 1;
        }
        Ok(())
    }

    /// addBlockIndices appends a range of indices (from
    /// startIndex to endIndex, inclusive) to the
    /// blockIndices channel. When all indices are added,
    /// the channel is closed.
    fn add_block_indices(
        &self,
        context: Context<SyncerError>,
        block_indices: Sender<i64>,
        start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        let mut i = start_index;
        while i <= end_index {
            // Don't load if we already have a healthy backlog.
            if block_indices.len() > *self.concurrency.lock() {
                sleep(DEFAULT_FETCH_SLEEP);
                continue;
            }

            context.err()?;
            block_indices.send(i).unwrap();
            i += 1;
        }

        // We populate doneLoading before exiting
        // to make sure we don't create more goroutines
        // when we are done. If we don't do this, we may accidentally
        // try to create a new goroutine after Wait has returned.
        // This will cause a panic.
        *self.done_loading.lock() = true;

        Ok(())
    }

    #[allow(clippy::missing_docs_in_private_items)]
    fn fetch_block_result(
        &self,
        context: &Context<SyncerError>,
        index: i64,
    ) -> SyncerResult<BlockResult> {
        let block = self.helper.block(
            // mock structures inside syncer_tests require access to syncer during this method for certain checks
            #[cfg(test)]
            self,
            context,
            &self.network,
            &PartialBlockIdentifier {
                index: Some(index),
                ..Default::default()
            },
        );

        let br = BlockResult {
            index,
            orphaned_head: match block {
                Ok(_) => false,
                Err(SyncerError::OrphanHead) => true,
                Err(e) => return Err(format!("unable to fetch block {}: {}", index, e).into()),
            },
            block: block.ok().flatten(),
        };

        self.handle_seen_block(context, &br).map_err(|e| {
            format!(
                "failed to handle the event of block {} is seen: {e}",
                br.index
            )
        })?;

        Ok(br)
    }

    /// spawns a new fetcher thread
    pub fn spawn_fetcher(
        &mut self,
        thread_handler: &mut ThreadHandler<(), SyncerError>,
        block_indices_receiver: Receiver<i64>,
        fetched_blocks_sender: Arc<Sender<BlockResult>>,
    ) {
        let context = thread_handler.ctx().clone();
        let syncer = self.clone();
        thread_handler.push(spawn(move || {
            *syncer.concurrency.lock() += 1;
            let res = syncer.fetch_blocks(&context, &block_indices_receiver, fetched_blocks_sender);
            *syncer.concurrency.lock() -= 1;
            res
        }))
    }

    /// fetchBlocks fetches blocks from a
    /// channel with retries until there are no
    /// more blocks in the channel or there is an
    /// error.
    fn fetch_blocks(
        &self,
        context: &Context<SyncerError>,
        block_indices_receiver: &Receiver<i64>,
        fetched_blocks_sender: Arc<Sender<BlockResult>>,
    ) -> SyncerResult<()> {
        for b in block_indices_receiver {
            let br = self
                .fetch_block_result(context, b)
                .map_err(|e| format!("unable to fetch block {b}: {e}"))?;

            context.err()?;
            fetched_blocks_sender.send(br).unwrap();

            // Exit if concurrency is greater than
            // goal concurrency.
            if *self.concurrency.lock() > *self.goal_concurrency.lock() {
                break;
            }
        }
        Ok(())
    }

    /// processBlocks is invoked whenever a new block is fetched. It attempts
    /// to process as many blocks as possible.
    fn process_blocks(
        &mut self,
        context: &Context<SyncerError>,
        cache: &mut IndexMap<i64, BlockResult>,
        end_index: i64,
    ) -> SyncerResult<()> {
        // We need to determine if we are in a reorg
        // so that we can force blocks to be fetched
        // if they don't exist in the cache.
        let mut reorg_start = -1;

        while self.next_index <= end_index {
            let br = if let Some(br) = cache.remove(&self.next_index) {
                // Anytime we re-fetch an index, we
                // will need to make another call to the node
                // as it is likely in a reorg.
                br
            } else {
                // Wait for more blocks if we aren't
                // in a reorg.
                if reorg_start < self.next_index {
                    break;
                }

                // Fetch the nextIndex if we are
                // in a re-org.
                self.fetch_block_result(context, self.next_index)
                    .map_err(|e| {
                        format!(
                            "unable to fetch block {} during re-org: {e}",
                            self.next_index
                        )
                    })?
            };

            let last_processed = self.next_index;
            let br_index = br.index;
            self.process_block(context, Some(br))
                .map_err(|e| format!("unable to process block {br_index}: {e}"))?;

            if self.next_index < last_processed && reorg_start == -1 {
                reorg_start = last_processed
            }
        }

        Ok(())
    }

    #[allow(clippy::missing_docs_in_private_items)]
    fn adjust_workers(&mut self, concurrency: &mut MutexGuard<usize>) -> bool {
        // find max block size
        let max = self
            .recent_block_sizes
            .iter()
            .max()
            .copied()
            .unwrap_or_default() as f64
            * self.size_multiplier;

        // Check if we have entered shutdown
        // and return false if we have.
        if **concurrency == 0 {
            return false;
        }

        // multiply average block size by concurrency
        let estimated_max_cache = max * **concurrency as f64;

        // If < cacheSize, increase concurrency by 1 up to MaxConcurrency
        let should_create = if estimated_max_cache + max < self.cache_size as f64
            && **concurrency < self.max_concurrency
            && self.last_adjustment > self.adjustment_window
        {
            let mut goal_concurrency = self.goal_concurrency.lock();
            *goal_concurrency += 1;
            self.last_adjustment = 0;
            tracing::info!(
                "increasing syncer concurrency to {} (projected new cache size: {} MB)\n",
                *goal_concurrency,
                // TODO should be b_to_mb function inside utils crate
                max * *goal_concurrency as f64 / 1024.0 / 1024.0
            );
            true
        } else {
            false
        };

        // If >= cacheSize, decrease concurrency however many necessary to fit max cache size.
        //
        // Note: We always will decrease size, regardless of last adjustment.
        if estimated_max_cache > self.cache_size as f64 {
            let mut new_goal_concurrency = (self.cache_size as f64 / max) as usize;
            if new_goal_concurrency < MIN_CONCURRENCY {
                new_goal_concurrency = MIN_CONCURRENCY
            }

            // Only log if s.goalConcurrency != newGoalConcurrency
            let mut goal_concurrency = self.goal_concurrency.lock();
            if *goal_concurrency != new_goal_concurrency {
                *goal_concurrency = new_goal_concurrency;
                self.last_adjustment = 0;
                tracing::info!(
                    "reducing syncer concurrency to {} (projected new cache size: {} MB)\n",
                    *goal_concurrency,
                    // TODO should be b_to_mb function inside utils crate
                    max * *goal_concurrency as f64 / 1024.0 / 1024.0
                )
            }
        }

        if self.recent_block_sizes.len() > DEFAULT_TRAILING_WINDOW {
            self.recent_block_sizes.pop_front();
        }

        should_create
    }

    #[allow(clippy::missing_docs_in_private_items)]
    fn handle_seen_block(
        &self,
        context: &Context<SyncerError>,
        result: &BlockResult,
    ) -> SyncerResult<()> {
        // If the helper returns ErrOrphanHead
        // for a block fetch, result.block will
        // be nil.
        if let Some(b) = &result.block {
            self.handler.block_seen(context, b)
        } else {
            Ok(())
        }
    }

    #[allow(clippy::missing_docs_in_private_items, clippy::too_many_arguments)]
    fn sequence_blocks(
        &mut self,
        thread_handler: &mut ThreadHandler<(), SyncerError>,
        block_indices_receiver: Receiver<i64>,
        fetched_blocks_sender: Arc<Sender<BlockResult>>,
        fetched_blocks_receiver: Receiver<BlockResult>,
        end_index: i64,
    ) -> SyncerResult<()> {
        let weak_fetched_blocks_sender = Arc::downgrade(&fetched_blocks_sender);
        drop(fetched_blocks_sender);
        let mut cache = IndexMap::new();
        for result in fetched_blocks_receiver {
            let size = result.estimated_size();
            cache.insert(result.index, result);

            self.process_blocks(thread_handler.ctx(), &mut cache, end_index)
                .map_err(|e| {
                    format!(
                        "unable to process block range {}-{end_index}: {e}",
                        self.next_index
                    )
                })?;

            // Determine if concurrency should be adjusted.
            self.recent_block_sizes.push_back(size);
            self.last_adjustment += 1;

            thread_handler.update();

            let tmp_concurrency = self.concurrency.clone();
            let mut concurrency = tmp_concurrency.lock();
            let should_create = self.adjust_workers(&mut concurrency);
            if !should_create {
                continue;
            }

            // If we have finished loading blocks or the pipelineCtx
            // has an error (like context.Canceled), we should avoid
            // creating more goroutines (as there is a chance that
            // Wait has returned). Attempting to create more goroutines
            // after Wait has returned will cause a panic.\
            if !*self.done_loading.lock() && !thread_handler.ctx().done() {
                let sender = match weak_fetched_blocks_sender.upgrade() {
                    Some(s) => s,
                    None => break,
                };
                self.spawn_fetcher(thread_handler, block_indices_receiver.clone(), sender)
            }

            // // Hold concurrencyLock until after we attempt to create another
            // // new goroutine in the case we accidentally go to 0 during shutdown.
            drop(concurrency);
        }

        Ok(())
    }

    /// syncRange fetches and processes a range of blocks
    /// (from syncer.nextIndex to endIndex, inclusive)
    /// with syncer.concurrency.
    fn sync_range(&mut self, context: &Context<SyncerError>, end_index: i64) -> SyncerResult<()> {
        let (block_indices_sender, block_indices_receiver) = unbounded();
        let (fetched_blocks_sender, fetched_blocks_receiver) = {
            let (s, r) = unbounded();
            (Arc::new(s), r)
        };

        // Ensure starting concurrency is less than max concurrency.
        // Don't create more goroutines than there are blocks
        // to sync.
        let starting_concurrency = [
            self.max_concurrency,
            DEFAULT_CONCURRENCY,
            (end_index - self.next_index + 1).try_into().unwrap(),
        ]
        .into_iter()
        .min()
        .unwrap();

        // Reset sync variables
        self.recent_block_sizes.clear();
        self.last_adjustment = 0;
        *self.done_loading.lock() = false;
        *self.concurrency.lock() = 0;
        *self.goal_concurrency.lock() = starting_concurrency;

        let mut thread_handler = ThreadHandler::from(context.clone());

        let tmp_self = self.clone();
        let tmp_context = thread_handler.ctx().clone();
        thread_handler.push(spawn(move || {
            tmp_self.add_block_indices(
                tmp_context,
                block_indices_sender,
                tmp_self.next_index,
                end_index,
            )
        }));

        for _ in 0..starting_concurrency {
            self.spawn_fetcher(
                &mut thread_handler,
                block_indices_receiver.clone(),
                fetched_blocks_sender.clone(),
            )
        }

        self.sequence_blocks(
            &mut thread_handler,
            block_indices_receiver,
            fetched_blocks_sender,
            fetched_blocks_receiver,
            end_index,
        )
        .map_err(|e| {
            format!(
                "failed to sequence block range {}-{end_index}: {e}",
                self.next_index
            )
        })?;

        // Wait for all block fetching threads to exit
        thread_handler.wait();
        context.err()
    }

    /// Tip returns the last observed tip. The tip is recorded
    /// at the start of each sync range and should only be thought
    /// of as a best effort approximation of tip.
    ///
    /// This can be very helpful to callers who want to know
    /// an approximation of tip very frequently (~every second)
    /// but don't want to implement their own caching logic.
    pub fn tip(&self) -> Option<&BlockIdentifier> {
        self.tip.as_ref()
    }

    /// Sync cycles endlessly until there is an error
    /// or the requested range is synced. When the requested
    /// range is synced, context is canceled.
    pub fn sync(
        &mut self,
        context: &Context<SyncerError>,
        mut start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        self.set_start(context, start_index)
            .map_err(|e| format!("unable to set start index {start_index}: {e}"))?;
        loop {
            let (range_end, halt) = self
                .next_syncable_range(context, end_index)
                .map_err(|e| format!("unable to get next syncable range: {e}"))?;

            if halt {
                if self.next_index > end_index && end_index != -1 {
                    break;
                } else {
                    sleep(DEFAULT_SYNC_SLEEP);
                    continue;
                }
            }

            if self.next_index != range_end {
                tracing::info!("Syncing {}-{}\n", self.next_index, range_end);
            } else {
                tracing::info!("Syncing {}\n", self.next_index);
            }

            self.sync_range(context, range_end)?;
        }

        if start_index == -1 {
            start_index = self.genesis_block.as_ref().unwrap().index;
        }

        if self.cancel {
            context.cancel();
        }
        tracing::info!("Finished syncing {}-{}\n", start_index, end_index);
        Ok(())
    }
}
