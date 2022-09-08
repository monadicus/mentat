use std::hash::Hasher;

use fnv::FnvHasher;
use indexmap::IndexMap;

use crate::priority_mutex::{PriorityMutex, PriorityMutexGuard};

/// shardMapEntry governs access to the shard of
/// the map contained at a particular index.
pub type ShardedMapEntry<T> = PriorityMutex<IndexMap<String, T>>;

/// ShardedMap allows concurrent writes
/// to a map by sharding the map into some
/// number of independently locked subsections.
pub struct ShardedMap<T>(Vec<ShardedMapEntry<T>>);

impl<T> ShardedMap<T> {
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

    /// shardIndex returns the index of the shard
    /// that could contain the key.
    pub fn shard_index(&self, key: &str) -> usize {
        // TODO go code uses FNV-1a but were using FNV-1
        let mut hasher = FnvHasher::default();
        hasher.write(key.as_bytes());
        hasher.finish() as usize % self.0.len()
    }

    /// Lock acquires the lock for a shard that could contain
    /// the key. This syntax allows the caller to perform multiple
    /// operations while holding the lock for a single shard.
    pub fn lock(&self, key: &str, priority: bool) -> PriorityMutexGuard<IndexMap<String, T>> {
        let shard_index = self.shard_index(key);
        self.0[shard_index].lock(priority)
    }
}

impl<T: Default> Default for ShardedMap<T> {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SHARDS)
    }
}
