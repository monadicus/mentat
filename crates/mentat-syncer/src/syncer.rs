//! TODO

use std::{
    mem::size_of_val,
    sync::Arc,
    thread::{sleep, spawn, JoinHandle},
};

use crossbeam_channel::{unbounded, Receiver, SendError, Sender};
use indexmap::IndexMap;
use mentat_types::{
    hash, Block, BlockIdentifier, EstimateSize, NetworkIdentifier, PartialBlockIdentifier,
};
use parking_lot::{Mutex, MutexGuard};

use crate::{
    errors::{SyncerError, SyncerResult},
    types::{
        ErrorBuf, Handler, Helper, Syncer, DEFAULT_CONCURRENCY, DEFAULT_FETCH_SLEEP,
        DEFAULT_SYNC_SLEEP, DEFAULT_TRAILING_WINDOW, MIN_CONCURRENCY,
    },
};

/// macro to aid in cloning values for threads
macro_rules! enclose {
    (($($i:ident),*) $y:expr) => {{
        $(let $i = $i.clone();)*
        $y
    }}
}

/// sender channel that supports hanging up
#[derive(Clone)]
struct ArcSender<T>(Arc<Mutex<Option<Sender<T>>>>);

#[allow(clippy::missing_docs_in_private_items)]
impl<T> ArcSender<T> {
    fn spawn() -> (Self, Receiver<T>) {
        let (sender, receiver) = unbounded();
        (Self(Arc::new(Mutex::new(Some(sender)))), receiver)
    }

    fn send(&self, payload: T) -> Option<Result<(), SendError<T>>> {
        self.0.lock().as_ref().map(|s| s.send(payload))
    }

    fn close(self) {
        *self.0.lock() = None;
    }
}

/// blockResult is returned by calls
/// to fetch a particular index. We must
/// use a separate index field in case
/// the block is omitted and we can't
/// determine the index of the request.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Default, Clone)]
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
    fn set_start(&mut self, error_buf: &ErrorBuf, index: i64) -> SyncerResult<()> {
        let network_status = self.helper.network_status(error_buf, &self.network)?;
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
        error_buf: &ErrorBuf,
        mut end_index: i64,
    ) -> SyncerResult<(i64, bool)> {
        if self.next_index == -1 {
            return Err(SyncerError::GetCurrentHeadBlockFailed);
        }

        // Always fetch network status to ensure endIndex is not past tip
        let network_status = self
            .helper
            .network_status(error_buf, &self.network)
            .map_err(|e| format!("{}: {}", SyncerError::GetNetworkStatusFailed, e))?;

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
    pub fn process_block(
        &mut self,
        error_buf: &ErrorBuf,
        br: Option<BlockResult>,
    ) -> SyncerResult<()> {
        let br = br.ok_or(SyncerError::BlockResultNil)?;
        if br.block.is_none() && !br.orphaned_head {
            // If the block is omitted, increase
            // index and return.
            self.next_index += 1;
            Ok(())
        } else if let (true, last_block) = self.check_remove(&br)? {
            self.handler.block_removed(error_buf, last_block)?;
            self.next_index = last_block.unwrap().index;
            self.past_blocks.pop_back();
            Ok(())
        } else {
            // mock structures inside syncer_tests require access to syncer during this method for certain checks
            #[cfg(test)]
            self.handler
                .block_added(self, error_buf, br.block.clone())?;
            #[cfg(not(test))]
            self.handler.block_added(error_buf, br.block.as_ref())?;
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
    fn add_block_indices(
        &self,
        error_buf: ErrorBuf,
        block_indices: Sender<i64>,
        start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        let mut i = start_index;
        while i <= end_index {
            // Don't load if we already have a healthy backlog.
            if block_indices.len() as i64 > *self.concurrency.lock() {
                sleep(DEFAULT_FETCH_SLEEP);
                continue;
            }

            if let Some(e) = &*error_buf.lock() {
                self.safe_exit(Err(e.clone()), &error_buf)?;
            } else {
                block_indices.send(i).unwrap();
                i += 1;
            }
        }

        // TODO suppose to hold done_loading lock til after the defer
        //
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
        error_buf: &ErrorBuf,
        network: &NetworkIdentifier,
        index: i64,
    ) -> SyncerResult<BlockResult> {
        let block = self.helper.block(
            // mock structures inside syncer_tests require access to syncer during this method for certain checks
            #[cfg(test)]
            self,
            error_buf,
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
            block: block.ok().flatten(),
        };

        self.handle_seen_block(error_buf, &br)?;

        Ok(br)
    }

    /// safeExit ensures we lower the concurrency in a lock while
    /// exiting. This prevents us from accidentally increasing concurrency
    /// when we are shutting down.
    fn safe_exit<T>(&self, res: SyncerResult<T>, buf: &ErrorBuf) -> SyncerResult<T> {
        if let Err(e) = &res {
            *buf.lock() = Some(e.clone())
        }
        *self.concurrency.lock() -= 1;
        res
    }

    /// fetchBlocks fetches blocks from a
    /// channel with retries until there are no
    /// more blocks in the channel or there is an
    /// error.
    fn fetch_blocks(
        &self,
        error_buf: ErrorBuf,
        network: &NetworkIdentifier,
        block_indices: Receiver<i64>,
        results: ArcSender<BlockResult>,
    ) -> SyncerResult<()> {
        for b in block_indices {
            let br = match self
                .fetch_block_result(&error_buf, network, b)
                .map_err(|e| format!("{} {}: {}", SyncerError::FetchBlockFailed, b, e))
            {
                Ok(v) => v,
                Err(e) => self.safe_exit(Err(e.into()), &error_buf)?,
            };

            if let Some(e) = &*error_buf.lock() {
                self.safe_exit(Err(e.clone()), &error_buf)?;
            } else if let Some(v) = results.send(br) {
                v.unwrap()
            } else {
                self.safe_exit(Err(SyncerError::Cancelled), &error_buf)?
            }

            // Exit if concurrency is greater than
            // goal concurrency.
            let mut concurrency = self.concurrency.lock();
            if *concurrency > self.goal_concurrency {
                *concurrency -= 1;
                return Ok(());
            }
        }
        self.safe_exit(Ok(()), &error_buf)
    }

    /// processBlocks is invoked whenever a new block is fetched. It attempts
    /// to process as many blocks as possible.
    fn process_blocks(
        &mut self,
        error_buf: &ErrorBuf,
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
                self.fetch_block_result(error_buf, &self.network, self.next_index)
                    .map_err(|e| format!("{}: {}", SyncerError::FetchBlockFailed, e))?
            };

            let last_processed = self.next_index;
            self.process_block(error_buf, Some(br))
                .map_err(|e| format!("{}: {}", SyncerError::BlockProcessFailed, e))?;

            if self.next_index < last_processed && reorg_start == -1 {
                reorg_start = last_processed
            }
        }

        Ok(())
    }

    #[allow(clippy::missing_docs_in_private_items)]
    fn adjust_workers(&mut self, concurrency: &mut MutexGuard<i64>) -> bool {
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
            self.goal_concurrency += 1;
            **concurrency += 1;
            self.last_adjustment = 0;
            tracing::info!(
                "increasing syncer concurrency to {} (projected new cache size: {} MB)\n",
                self.goal_concurrency,
                // TODO should be b_to_mb function inside utils crate
                max * self.goal_concurrency as f64 / 1024.0 / 1024.0
            );
            true
        } else {
            false
        };

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
    fn handle_seen_block(&self, error_buf: &ErrorBuf, result: &BlockResult) -> SyncerResult<()> {
        // If the helper returns ErrOrphanHead
        // for a block fetch, result.block will
        // be nil.
        if let Some(b) = &result.block {
            self.handler.block_seen(error_buf, b)
        } else {
            Ok(())
        }
    }

    #[allow(clippy::missing_docs_in_private_items, clippy::too_many_arguments)]
    fn sequence_blocks(
        &mut self,
        error_buf: &ErrorBuf,
        pipeline_exit: &ErrorBuf,
        handles: &Arc<Mutex<Vec<JoinHandle<SyncerResult<()>>>>>,
        block_indices: Receiver<i64>,
        fetched_blocks_sender: ArcSender<BlockResult>,
        fetched_blocks_receiver: Receiver<BlockResult>,
        end_index: i64,
    ) -> SyncerResult<()> {
        let mut cache = IndexMap::new();
        for result in fetched_blocks_receiver {
            // TODO make sure this returns full size and not just size of immediate pointer
            let size = result.estimated_size() as i64;
            cache.insert(result.index, result);

            self.process_blocks(error_buf, &mut cache, end_index)
                .map_err(|e| format!("{}: {}", SyncerError::BlocksProcessMultipleFailed, e))?;

            // Determine if concurrency should be adjusted.
            self.recent_block_sizes.push_back(size);
            self.last_adjustment += 1;

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
            // after Wait has returned will cause a panic.
            if !*self.done_loading.lock() && pipeline_exit.lock().is_none() {
                let tmp_self = self.clone();
                handles.lock().push(enclose!(
                    (pipeline_exit, block_indices, fetched_blocks_sender)
                    spawn(move || {
                        tmp_self.fetch_blocks(
                            pipeline_exit,
                            &tmp_self.network,
                            block_indices,
                            fetched_blocks_sender,
                        )
                    })
                ));
            } else {
                *concurrency -= 1;
            }

            // Hold concurrencyLock until after we attempt to create another
            // new goroutine in the case we accidentally go to 0 during shutdown.
            drop(concurrency);
        }

        Ok(())
    }

    /// Wait for all block fetching threads to exit
    /// before closing the fetched_blocks channel.
    fn waiter(
        handles: Arc<Mutex<Vec<JoinHandle<SyncerResult<()>>>>>,
        fetched_blocks: ArcSender<BlockResult>,
        error_buf: ErrorBuf,
    ) -> SyncerResult<()> {
        let res = loop {
            let mut handles = handles.lock();
            if handles.is_empty() {
                break Ok(());
            }

            if let Some(i) = handles.iter().position(|h| h.is_finished()) {
                match handles.remove(i).join() {
                    Ok(Ok(())) => continue,
                    Ok(Err(e)) => {
                        break Err(e);
                    }
                    Err(e) => {
                        let e = SyncerError::String(format!("{e:?}"));
                        break Err(e);
                    }
                }
            }
        };
        if let Err(e) = &res {
            *error_buf.lock() = Some(e.clone());
        }
        fetched_blocks.close();
        res
    }

    /// syncRange fetches and processes a range of blocks
    /// (from syncer.nextIndex to endIndex, inclusive)
    /// with syncer.concurrency.
    fn sync_range(&mut self, error_buf: &ErrorBuf, end_index: i64) -> SyncerResult<()> {
        let (block_indices_sender, block_indices_receiver) = unbounded();
        let (fetched_blocks_sender, fetched_blocks_receiver) = ArcSender::spawn();

        // Ensure default concurrency is less than max concurrency.
        // Don't create more goroutines than there are blocks
        // to sync.
        let starting_concurrency = [
            self.max_concurrency,
            DEFAULT_CONCURRENCY,
            end_index - self.next_index + 1,
        ]
        .into_iter()
        .min()
        .unwrap();

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
        let handles = Arc::new(Mutex::new(Vec::new()));
        let pipeline_buf = Arc::new(Mutex::new(None));

        let tmp_self = self.clone();
        let tmp_pipeline = pipeline_buf.clone();
        handles.lock().push(spawn(move || {
            tmp_self.add_block_indices(
                tmp_pipeline,
                block_indices_sender,
                tmp_self.next_index,
                end_index,
            )
        }));

        for _ in 0..starting_concurrency {
            let tmp_self = self.clone();
            handles.lock().push(enclose!(
                (pipeline_buf, block_indices_receiver, fetched_blocks_sender)
                spawn(move || {
                    tmp_self.fetch_blocks(pipeline_buf, &tmp_self.network, block_indices_receiver, fetched_blocks_sender)
                })
            ));
        }

        // Wait for all block fetching goroutines to exit
        // before closing the fetchedBlocks channel.
        let wait_handle = enclose!(
            (handles, fetched_blocks_sender, pipeline_buf)
            spawn(move || {
                Self::waiter(handles, fetched_blocks_sender, pipeline_buf)
            })
        );

        self.sequence_blocks(
            error_buf,
            &pipeline_buf,
            &handles,
            block_indices_receiver,
            fetched_blocks_sender,
            fetched_blocks_receiver,
            end_index,
        )?;

        // if a sub-thread panics then the main thread will panic too
        wait_handle
            .join()
            .unwrap()
            .map_err(|e| format!("{}: unable to sync to {}", e, end_index).into())
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
        error_buf: &ErrorBuf,
        mut start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        self.set_start(error_buf, start_index)
            .map_err(|e| format!("{}: {}", SyncerError::SetStartIndexFailed, e))?;
        loop {
            let (range_end, halt) = self
                .next_syncable_range(error_buf, end_index)
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

            self.sync_range(error_buf, range_end)
                .map_err(|e| format!("{}: unable to sync to {}", e, range_end))?;

            if let Some(e) = &*error_buf.lock() {
                return Err(e.clone());
            }
        }

        if start_index == -1 {
            start_index = self.genesis_block.as_ref().unwrap().index;
        }

        if self.cancel {
            *error_buf.lock() = Some(SyncerError::Cancelled);
        }
        tracing::info!("Finished syncing {}-{}\n", start_index, end_index);
        Ok(())
    }
}
