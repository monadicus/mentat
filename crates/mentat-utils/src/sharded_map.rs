use fnv::{FnvHashMap, FnvHashSet};
use indexmap::IndexSet;

use crate::priority_mutex::{PriorityMutex, PriorityMutexGuard};

/// shardMapEntry governs access to the shard of
/// the map contained at a particular index.
type ShardedMapEntry = PriorityMutex<IndexSet<String>>;

/// ShardedMap allows concurrent writes
/// to a map by sharding the map into some
/// number of independently locked subsections.
pub struct ShardedMap(Vec<ShardedMapEntry>);

impl ShardedMap {
    /// the default number of shards to use in `ShardedMap`
    const DEFAULT_SHARDS: usize = 256;

    /// NewShardedMap creates a new *ShardedMap
    /// with some number of shards. The larger the
    /// number provided for shards, the less lock
    /// contention there will be.
    ///
    /// As a rule of thumb, shards should usually
    /// be set to the concurrency of the caller.
    pub fn new(shards: usize) -> Self {
        Self(
            (0..shards)
                .into_iter()
                .map(|_| ShardedMapEntry::default())
                .collect(),
        )
    }

    // TODO figure out how to use the fnv crate to hash this with fnv1a. may need to store a hasher on Self
    /// shardIndex returns the index of the shard
    /// that could contain the key.
    pub fn shard_index(&self, key: String) -> usize {
        todo!()
    }

    /// Lock acquires the lock for a shard that could contain
    /// the key. This syntax allows the caller to perform multiple
    /// operations while holding the lock for a single shard.
    pub fn lock(&self, key: String, priority: bool) -> PriorityMutexGuard<IndexSet<String>> {
        todo!()
    }
}

impl Default for ShardedMap {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SHARDS)
    }
}
