//! TODO: figure out how to do this without breaking memory rules or causing
//! incorrect locking

use std::{ops::DerefMut, sync::Arc};

use parking_lot::{RwLock, RwLockWriteGuard};

use crate::{priority_mutex::PriorityMutex, sharded_map::ShardedMap};

/// The primitive used
/// to track claimed PriorityMutex
pub type MutexMapEntry<T> = Arc<PriorityMutex<T>>;

/// A struct that allows for
/// acquiring a [`PriorityMutex`] via a string identifier
/// or for acquiring a global mutex that blocks
/// the acquisition of any identifier mutexes.
///
/// This is useful for coordinating concurrent, non-overlapping
/// writes in the storage package.
pub struct MutexMap<T> {
    entries: RwLock<ShardedMap<MutexMapEntry<T>>>,
}

impl<T> MutexMap<T> {
    /// returns a new `MutexMap`.
    pub fn new(shards: usize) -> MutexMap<T> {
        Self {
            entries: RwLock::new(ShardedMap::new(shards)),
        }
    }

    /// acquires an exclusive lock across an entire `MutexMap`.
    /// NOTE: the behavior of this differs from the official go SDK.
    /// parking_lot uses a "fair" model for RWLocks where readers will block when a writer is trying to acquire a lock even if the read lock is free and the read lock was queued before the write lock.
    /// meanwhile go uses a greedy model, where readers will always be given priority over writers.
    pub fn global_lock(&self) -> RwLockWriteGuard<ShardedMap<MutexMapEntry<T>>> {
        self.entries.write()
    }

    /// acquires a lock for a particular identifier, as long
    /// as no other caller has the global mutex or a lock
    /// by the same identifier.
    pub fn inspect<F: FnOnce(&mut T)>(&self, identifier: &str, priority: bool, action: F)
    where
        T: Default,
    {
        // We acquire a RLock on m.globalMutex before
        // acquiring our identifier lock to ensure no
        // thread holds an identifier mutex while
        // the m.globalMutex is also held.
        let entries = self.entries.read();

        // We acquire m when adding items to m.table
        // so that we don't accidentally overwrite
        // lock created by another thread.
        let mut data = entries.lock(identifier, priority);
        if !data.contains_key(identifier) {
            data.insert(identifier.into(), Default::default());
        }

        let entry = data.get(identifier).unwrap();
        let mut value = entry.lock(priority);

        action(value.deref_mut());

        drop(value);

        if Arc::strong_count(entry) <= 1 {
            data.remove(identifier);
        }
    }
}
