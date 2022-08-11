//! TODO

use std::{
    borrow::BorrowMut,
    ops::Add,
    sync::mpsc::{Receiver, Sender},
};

use indexmap::IndexMap;
use mentat_types::{hash, Block, BlockIdentifier, NetworkIdentifier};
use tokio::select;

use crate::{
    errors::{SyncerError, SyncerResult},
    types::{Handler, Helper, Syncer},
};

/// blockResult is returned by calls
/// to fetch a particular index. We must
/// use a separate index field in case
/// the block is omitted and we can't
/// determine the index of the request.
#[allow(clippy::missing_docs_in_private_items)]
pub struct BlockResult<'a> {
    index: i64,
    block: &'a Block,
    orphaned_head: bool,
}

impl<'net_id, 'block_id, Hand: Handler, Help: Helper> Syncer<'net_id, 'block_id, Hand, Help> {
    #[allow(clippy::missing_docs_in_private_items)]
    pub fn set_start(&mut self, ctx: (), index: i64) -> SyncerResult<()> {
        let network_status = Help::network_status(ctx, self.network)?;
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
        ctx: (),
        mut end_index: i64,
    ) -> (i64, bool, SyncerResult<()>) {
        if self.next_index == -1 {
            return (-1, false, Err(SyncerError::GetCurrentHeadBlockFailed));
        }

        // Always fetch network status to ensure endIndex is not past tip
        let network_status = match Help::network_status(ctx, self.network) {
            Ok(v) => v,
            Err(e) => {
                return (
                    -1,
                    false,
                    Err(format!("{}: {}", SyncerError::GetNetworkStatusFailed, e).into()),
                )
            }
        };
        let current_block_identifier_index = network_status.current_block_identifier.index;

        // Update the syncer's known tip
        self.tip = network_status.current_block_identifier;
        if end_index == -1 || end_index > current_block_identifier_index {
            end_index = current_block_identifier_index
        }

        if self.next_index > end_index {
            (-1, true, Ok(()))
        } else {
            (end_index, false, Ok(()))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn attempt_orphan<'a>(
        &self,
        last_block: &'a BlockIdentifier,
    ) -> (bool, Option<&'a BlockIdentifier>, SyncerResult<()>) {
        if hash(Some(&self.genesis_block)) == hash(Some(last_block)) {
            (false, None, Err(SyncerError::CannotRemoveGenesisBlock))
        } else {
            (true, Some(last_block), Ok(()))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn check_remove<'a>(
        &'a mut self,
        br: &BlockResult,
    ) -> (bool, Option<&'a BlockIdentifier>, SyncerResult<()>) {
        if self.past_blocks.is_empty() {
            return (false, None, Ok(()));
        }

        let last_block = *self.past_blocks.last().unwrap();
        if br.orphaned_head {
            return self.attempt_orphan(last_block);
        }

        // Ensure processing correct index
        let block = &br.block;
        if block.block_identifier.index != self.next_index {
            (
                false,
                None,
                Err(format!(
                    "{}: got block {} instead of {}",
                    SyncerError::OutOfOrder,
                    block.block_identifier.index,
                    self.next_index
                )
                .into()),
            )
            // Check if block parent is head
        } else if hash(Some(&block.parent_block_identifier)) != hash(Some(last_block)) {
            self.attempt_orphan(last_block)
        } else {
            (false, Some(last_block), Ok(()))
        }
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn process_block(&mut self, ctx: (), br: &BlockResult) -> SyncerResult<()> {
        todo!()
    }

    /// addBlockIndices appends a range of indices (from
    /// startIndex to endIndex, inclusive) to the
    /// blockIndices channel. When all indices are added,
    /// the channel is closed.
    pub fn add_block_indices(
        &mut self,
        ctx: (),
        block_indices: Sender<i64>,
        start_index: i64,
        end_index: i64,
    ) -> SyncerResult<()> {
        todo!()
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn fetch_block_result(
        &mut self,
        ctx: (),
        network: &NetworkIdentifier,
        index: i64,
    ) -> SyncerResult<&BlockResult> {
        todo!()
    }

    /// safeExit ensures we lower the concurrency in a lock while
    /// exiting. This prevents us from accidentally increasing concurrency
    /// when we are shutting down.
    pub fn safe_exit<T>(&mut self, res: SyncerResult<T>) -> SyncerResult<T> {
        *self.concurrency.lock().unwrap() -= 1;
        res
    }

    /// fetchBlocks fetches blocks from a
    /// channel with retries until there are no
    /// more blocks in the channel or there is an
    /// error.
    pub async fn fetch_blocks<'a>(
        &mut self,
        ctx: (),
        network: &NetworkIdentifier,
        block_indices: Receiver<i64>,
        results: Sender<BlockResult<'a>>,
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

            // TODO green thread error
            // select!(
            //     v = results.send(br) => v.unwrap()
            //     ctx.done() => {
            //         return self.safe_exit(ctx.err())
            //     }
            // );

            // Exit if concurrency is greater than
            // goal concurrency.
            let mut concurrency = self.concurrency.lock().unwrap();
            if *concurrency > self.goal_concurrency {
                *concurrency -= 1;
                return Ok(());
            }
        }

        todo!();
        self.safe_exit(Ok(()))
    }

    /// processBlocks is invoked whenever a new block is fetched. It attempts
    /// to process as many blocks as possible.
    fn process_blocks(
        &mut self,
        ctx: (),
        mut cache: IndexMap<i64, &BlockResult>,
        end_index: i64,
    ) -> SyncerResult<()> {
        // We need to determine if we are in a reorg
        // so that we can force blocks to be fetched
        // if they don't exist in the cache.
        let reorg_start = -1;

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
                self.fetch_block_result(ctx, self.network, self.next_index)
                    .map_err(|e| format!("{}: {}", SyncerError::FetchBlockFailed, e))?
            };

            // TODO borrow error need to figure out what is actually referenced where
            // let last_processed = self.next_index;
            // self.process_block(ctx, br)
            //     .map_err(|e| format!("{}: {}", SyncerError::BlockProcessFailed, e))?;

            // if self.next_index < last_processed && reorg_start == -1 {
            //     reorg_start = last_processed
            // }
        }

        todo!();
        Ok(())
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn adjust_workers(&mut self) -> bool {
        todo!()
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn handle_seen_block(&mut self, ctx: (), result: &BlockResult) -> SyncerResult<()> {
        todo!()
    }

    #[allow(clippy::missing_docs_in_private_items)]
    pub fn sequence_blocks(
        &mut self,
        ctx: (),
        pipeline_ctx: (),
        g: (), // errgroup.Group
        blockIndices: Receiver<i64>,
        fetch_blocks: Receiver<i64>,
        end_index: i64,
    ) -> SyncerResult<()> {
        todo!()
    }

    /// syncRange fetches and processes a range of blocks
    /// (from syncer.nextIndex to endIndex, inclusive)
    /// with syncer.concurrency.
    pub fn sync_range(&mut self, ctx: (), end_index: i64) -> SyncerResult<()> {
        todo!()
    }

    /// Tip returns the last observed tip. The tip is recorded
    /// at the start of each sync range and should only be thought
    /// of as a best effort approximation of tip.
    ///
    /// This can be very helpful to callers who want to know
    /// an approximation of tip very frequently (~every second)
    /// but don't want to implement their own caching logic.
    pub fn tip(&mut self) -> &BlockIdentifier {
        todo!()
    }

    /// Sync cycles endlessly until there is an error
    /// or the requested range is synced. When the requested
    /// range is synced, context is canceled.
    pub fn sync(&mut self, ctx: (), start_index: i64, end_index: i64) -> SyncerResult<()> {
        todo!()
    }
}
