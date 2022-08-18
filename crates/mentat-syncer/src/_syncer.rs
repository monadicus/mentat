//! TODO

use std::{mem::size_of_val, thread::sleep};

use indexmap::IndexMap;
use mentat_types::{hash, Block, BlockIdentifier, NetworkIdentifier, PartialBlockIdentifier};
use tokio::select;

use crate::{
    errors::{SyncerError, SyncerResult},
    golang::{self, chan::Context, defer, Chan},
    types::{
        Handler, Helper, Syncer, DEFAULT_CONCURRENCY, DEFAULT_FETCH_SLEEP, DEFAULT_SYNC_SLEEP,
        DEFAULT_TRAILING_WINDOW, MIN_CONCURRENCY,
    },
};

/// blockResult is returned by calls
/// to fetch a particular index. We must
/// use a separate index field in case
/// the block is omitted and we can't
/// determine the index of the request.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Default)]
pub struct BlockResult {
    index: i64,
    block: Option<Block>,
    orphaned_head: bool,
}

impl<Hand: Handler, Help: Helper> Syncer<Hand, Help> {
    #[allow(clippy::missing_docs_in_private_items)]
    pub fn set_start(&mut self, ctx: &Context, index: i64) -> SyncerResult<()> {
        let network_status = Help::network_status(ctx, &self.network)?;
        self.genesis_block = network_status.genesis_block_identifier;
        if index != 0 {
            self.next_index = index
        } else {
            self.next_index = self.genesis_block.index
        }
        Ok(())
    }

    /// nextSyncableRange returns the next range of indexes to sync
    /// based on what the last processed block in storage is and
    /// the contents of the network status response.
    pub fn next_syncable_range(
        &mut self,
        ctx: &Context,
        mut end_index: i64,
    ) -> SyncerResult<(i64, bool)> {
        if self.next_index == -1 {
            return Err(SyncerError::GetCurrentHeadBlockFailed);
        }

        // Always fetch network status to ensure endIndex is not past tip
        let network_status = match Help::network_status(ctx, &self.network) {
            Ok(v) => v,
            Err(e) => return Err(format!("{}: {}", SyncerError::GetNetworkStatusFailed, e).into()),
        };
        let current_block_identifier_index = network_status.current_block_identifier.index;

        // Update the syncer's known tip
        self.tip = network_status.current_block_identifier;
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
    pub fn attempt_orphan<'a>(
        &self,
        last_block: &'a BlockIdentifier,
    ) -> SyncerResult<(bool, Option<&'a BlockIdentifier>)> {
        if hash(Some(&self.genesis_block)) == hash(Some(last_block)) {
            Err(SyncerError::CannotRemoveGenesisBlock)
        } else {
            Ok((true, Some(last_block)))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn check_remove<'a>(
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

        // TODO: no nil check?
        // Ensure processing correct index
        let block = br.block.as_ref().unwrap();
        if block.block_identifier.index != self.next_index {
            Err(format!(
                "{}: got block {} instead of {}",
                SyncerError::OutOfOrder,
                block.block_identifier.index,
                self.next_index
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
    pub fn process_block(&mut self, ctx: &Context, br: Option<BlockResult>) -> SyncerResult<()> {
        let br = br.ok_or(SyncerError::BlockResultNil)?;
        if br.block.is_none() && !br.orphaned_head {
            // If the block is omitted, increase
            // index and return.
            self.next_index += 1;
            Ok(())
        } else if let (true, last_block) = self.check_remove(&br)? {
            Hand::block_removed(ctx, last_block)?;
            // TODO: no nil check? probably happens in block_removed
            self.next_index = last_block.unwrap().index;
            self.past_blocks.pop_back();
            Ok(())
        } else {
            Hand::block_added(ctx, br.block.as_ref())?;
            // TODO: no nil check? probably happens in block_added
            let block = br.block.unwrap();
            let idx = block.block_identifier.index;
            self.past_blocks.push_back(block.block_identifier);
            if self.past_blocks.len() > self.past_block_limit as usize {
                self.past_blocks.pop_front();
            }
            self.next_index = idx + 1;
            Ok(())
        }
    }

    /// addBlockIndices appends a range of indices (from
    /// startIndex to endIndex, inclusive) to the
    /// blockIndices channel. When all indices are added,
    /// the channel is closed.
    pub async fn add_block_indices(
        &self,
        ctx: &Context,
        block_indices: &Chan<i64>,
        start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        defer!(block_indices.close_s());

        for i in start_index..end_index {
            // Don't load if we already have a healthy backlog.
            if block_indices.tx().len() as i64 > *self.concurrency.lock() {
                sleep(DEFAULT_FETCH_SLEEP);
                continue;
            }

            select! {
                v = block_indices.send(i) => v.unwrap(),
                v = ctx.done() => return self.safe_exit(ctx.err())
            }
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
    pub fn fetch_block_result(
        &self,
        ctx: &Context,
        network: &NetworkIdentifier,
        index: i64,
    ) -> SyncerResult<BlockResult> {
        let block = Help::block(
            ctx,
            network,
            &PartialBlockIdentifier {
                index: Some(index),
                ..Default::default()
            },
        );

        let br = BlockResult {
            index,
            orphaned_head: match block {
                Ok(_) => false,
                Err(SyncerError::OrphanedHead) => true,
                Err(e) => return Err(e),
            },
            block: block.ok(),
        };

        self.handle_seen_block(ctx, &br)?;

        Ok(br)
    }

    /// safeExit ensures we lower the concurrency in a lock while
    /// exiting. This prevents us from accidentally increasing concurrency
    /// when we are shutting down.
    pub fn safe_exit<T>(&self, res: SyncerResult<T>) -> SyncerResult<T> {
        *self.concurrency.lock() -= 1;
        res
    }

    /// fetchBlocks fetches blocks from a
    /// channel with retries until there are no
    /// more blocks in the channel or there is an
    /// error.
    pub async fn fetch_blocks(
        &self,
        ctx: &Context,
        network: &NetworkIdentifier,
        block_indices: &Chan<i64>,
        results: &Chan<BlockResult>,
    ) -> SyncerResult<()> {
        for b in block_indices {
            let br = match self.fetch_block_result(ctx, network, b) {
                Ok(v) => v,
                Err(e) => {
                    return self.safe_exit(Err(format!(
                        "{} {}: {}",
                        SyncerError::FetchBlockFailed,
                        b,
                        e
                    )
                    .into()))
                }
            };

            select!(
                v = results.send(br) => v.unwrap(),
                v = ctx.done() => return self.safe_exit(ctx.err())
            );

            // Exit if concurrency is greater than
            // goal concurrency.
            let mut concurrency = self.concurrency.lock();
            if *concurrency > self.goal_concurrency {
                *concurrency -= 1;
                return Ok(());
            }
        }

        self.safe_exit(Ok(()))
    }

    /// processBlocks is invoked whenever a new block is fetched. It attempts
    /// to process as many blocks as possible.
    fn process_blocks(
        &mut self,
        ctx: &Context,
        cache: &mut IndexMap<i64, BlockResult>,
        end_index: i64,
    ) -> SyncerResult<()> {
        // We need to determine if we are in a reorg
        // so that we can force blocks to be fetched
        // if they don't exist in the cache.
        let mut reorg_start = -1;

        while self.next_index <= end_index {
            let br = if cache.contains_key(&self.next_index) {
                // Anytime we re-fetch an index, we
                // will need to make another call to the node
                // as it is likely in a reorg.
                cache.remove(&self.next_index).unwrap()
            } else {
                // Wait for more blocks if we aren't
                // in a reorg.
                if reorg_start < self.next_index {
                    break;
                }

                // Fetch the nextIndex if we are
                // in a re-org.
                self.fetch_block_result(ctx, &self.network, self.next_index)
                    .map_err(|e| format!("{}: {}", SyncerError::FetchBlockFailed, e))?
            };

            let last_processed = self.next_index;
            self.process_block(ctx, Some(br))
                .map_err(|e| format!("{}: {}", SyncerError::BlockProcessFailed, e))?;

            if self.next_index < last_processed && reorg_start == -1 {
                reorg_start = last_processed
            }
        }

        Ok(())
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn adjust_workers(&mut self) -> bool {
        // find max block size
        let max = self
            .recent_block_sizes
            .iter()
            .max()
            .copied()
            .unwrap_or_default() as f64
            * self.size_multiplier;

        let concurrency = *self.concurrency.lock() as i64;
        // Check if we have entered shutdown
        // and return false if we have.
        if concurrency == 0 {
            return false;
        }

        // multiply average block size by concurrency
        let estimated_max_cache = max * concurrency as f64;

        // If < cacheSize, increase concurrency by 1 up to MaxConcurrency
        let mut should_create = false;
        if estimated_max_cache + max < self.cache_size as f64
            && concurrency < self.max_concurrency
            && self.last_adjustment > self.adjustment_window
        {
            self.goal_concurrency += 1;
            *self.concurrency.lock() += 1;
            self.last_adjustment = 0;
            should_create = true;
            tracing::info!(
                "increasing syncer concurrency to {} (projected new cache size: {} MB)\n",
                self.goal_concurrency,
                // TODO should be b_to_mb function inside utils crate
                max * self.goal_concurrency as f64 / 1024.0 / 1024.0
            )
        }

        // If >= cacheSize, decrease concurrency however many necessary to fit max cache size.
        //
        // Note: We always will decrease size, regardless of last adjustment.
        if estimated_max_cache > self.cache_size as f64 {
            let mut new_goal_concurrency = (self.cache_size as f64 / max) as i64;
            if new_goal_concurrency < MIN_CONCURRENCY {
                new_goal_concurrency = MIN_CONCURRENCY
            }

            // Only log if s.goalConcurrency != newGoalConcurrency
            if self.goal_concurrency != new_goal_concurrency {
                self.goal_concurrency = new_goal_concurrency;
                self.last_adjustment = 0;
                tracing::info!(
                    "reducing syncer concurrency to %{}(projected new cache size: {} MB)\n",
                    self.goal_concurrency,
                    // TODO should be b_to_mb function inside utils crate
                    max * self.goal_concurrency as f64 / 1024.0 / 1024.0
                )
            }
        }

        if self.recent_block_sizes.len() as u64 > DEFAULT_TRAILING_WINDOW {
            self.recent_block_sizes.pop_front();
        }

        should_create
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn handle_seen_block(&self, ctx: &Context, result: &BlockResult) -> SyncerResult<()> {
        // If the helper returns ErrOrphanHead
        // for a block fetch, result.block will
        // be nil.
        if let Some(b) = &result.block {
            Hand::block_seen(ctx, b)
        } else {
            Ok(())
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn sequence_blocks(
        &mut self,
        ctx: &Context,
        pipeline_ctx: &Context,
        g: (), // errgroup.Group
        block_indices: &Chan<i64>,
        fetched_blocks: &Chan<BlockResult>,
        end_index: i64,
    ) -> SyncerResult<()> {
        let mut cache = IndexMap::new();
        for result in fetched_blocks {
            let size = size_of_val(&result) as i64;
            cache.insert(result.index, result);

            self.process_blocks(ctx, &mut cache, end_index)
                .map_err(|e| format!("{}: {}", SyncerError::BlocksProcessMultipleFailed, e))?;

            // Determine if concurrency should be adjusted.
            self.recent_block_sizes.push_back(size);
            self.last_adjustment += 1;

            // TODO: their adjust_workers fn assumes concurrencyLock is locked and uses concurrency freely
            // TODO: then they maintain this lock until the end of the function to prevent concurrency from being set to 0 by accident?
            // TODO: what is this thread-locking hackery??

            let tmp = self.concurrency.lock();
            let should_create = self.adjust_workers();
            if !should_create {
                drop(tmp);
                continue;
            }

            // TODO: need ctx and need to spawn new thread and need to impl bi-directional channels
            //
            // If we have finished loading blocks or the pipelineCtx
            // has an error (like context.Canceled), we should avoid
            // creating more goroutines (as there is a chance that
            // Wait has returned). Attempting to create more goroutines
            // after Wait has returned will cause a panic.
            if !*self.done_loading.lock() && pipeline_ctx.err().is_none() {
                g.go(|| {
                    return self.fetch_blocks(
                        pipeline_ctx,
                        &self.network,
                        block_indices,
                        fetched_blocks,
                    );
                })
            } else {
                *self.concurrency.lock() -= 1;
            }

            // TODO: this is where they drop the concurrencyLock
            //
            // Hold concurrencyLock until after we attempt to create another
            // new goroutine in the case we accidentally go to 0 during shutdown.
            self.concurrency.unlock();
        }

        Ok(())
    }

    /// syncRange fetches and processes a range of blocks
    /// (from syncer.nextIndex to endIndex, inclusive)
    /// with syncer.concurrency.
    pub fn sync_range(&mut self, ctx: &Context, end_index: i64) -> SyncerResult<()> {
        let mut block_indices = golang::make_unbounded();
        let mut fetched_blocks = golang::make_unbounded();

        // Ensure default concurrency is less than max concurrency.
        let mut starting_concurrency = if self.max_concurrency < DEFAULT_CONCURRENCY {
            self.max_concurrency
        } else {
            DEFAULT_CONCURRENCY
        };

        // Don't create more goroutines than there are blocks
        // to sync.
        let blocks_to_sync = end_index - self.next_index + 1;
        if blocks_to_sync < starting_concurrency {
            starting_concurrency = blocks_to_sync
        }

        // Reset sync variables
        self.recent_block_sizes.clear();
        self.last_adjustment = 0;
        *self.done_loading.lock() = false;
        *self.concurrency.lock() = starting_concurrency;
        self.goal_concurrency = starting_concurrency;

        // We create a separate derivative context here instead of
        // replacing the provided ctx because the context returned
        // by errgroup.WithContext is canceled as soon as Wait returns.
        // If this canceled context is passed to a handler or helper,
        // it can have unintended consequences (some functions
        // return immediately if the context is canceled).
        //
        // Source: https://godoc.org/golang.org/x/sync/errgroup
        // let (g, pipeline_ctx) = errgroup.with_context(ctx);
        let (g, pipeline_ctx) = todo!();
        g.go(|| {
            self.add_block_indices(
                pipeline_ctx,
                &block_indices.sender,
                self.next_index,
                end_index,
            )
        });

        for j in 0..starting_concurrency {
            g.go(|| {
                self.fetch_blocks(
                    pipeline_ctx,
                    &self.network,
                    &block_indices.receiver,
                    &fetched_blocks.sender,
                )
            })
        }

        // TODO this is not correct
        // Wait for all block fetching goroutines to exit
        // before closing the fetchedBlocks channel.
        let func = || {
            g.wait();
            drop(fetched_blocks)
        }();

        // TODO: context and errgroup
        self.sequence_blocks(
            ctx,
            pipeline_ctx,
            g,
            &block_indices,
            &fetched_blocks,
            end_index,
        )?;

        g.wait()
            .map_err(|e| format!("{}: unable to sync to {}", e, end_index))?;

        Ok(())
    }

    /// Tip returns the last observed tip. The tip is recorded
    /// at the start of each sync range and should only be thought
    /// of as a best effort approximation of tip.
    ///
    /// This can be very helpful to callers who want to know
    /// an approximation of tip very frequently (~every second)
    /// but don't want to implement their own caching logic.
    pub fn tip(&self) -> &BlockIdentifier {
        &self.tip
    }

    /// Sync cycles endlessly until there is an error
    /// or the requested range is synced. When the requested
    /// range is synced, context is canceled.
    pub fn sync(
        &mut self,
        ctx: &Context,
        mut start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        self.set_start(ctx, start_index)
            .map_err(|e| format!("{}: {}", SyncerError::SetStartIndexFailed, e))?;
        loop {
            let (range_end, halt) = self
                .next_syncable_range(ctx, end_index)
                .map_err(|e| format!("{}: {}", SyncerError::NextSyncableRangeFailed, e))?;

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

            self.sync_range(ctx, range_end)
                .map_err(|e| format!("{}: unable to sync to {}", e, range_end))?;

            if ctx.err().is_some() {
                return ctx.err();
            }
        }

        if start_index == -1 {
            start_index = self.genesis_block.index;
        }

        self.cancel();
        tracing::info!("Finished syncing {}-{}\n", start_index, end_index);
        Ok(())
    }
}
