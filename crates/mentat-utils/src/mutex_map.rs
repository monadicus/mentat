// use std::{
//     ops::{Deref, DerefMut},
//     rc::Rc,
//     sync::{atomic::AtomicBool, Arc},
// };

// use indexmap::IndexMap;
// use parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

// use crate::{
//     priority_mutex::{PriorityMutex, PriorityMutexGuard},
//     sharded_map::{ShardedMap, ShardedMapEntry},
// };

// const UNLOCK_PRIORITY: bool = true;

// /// mutexMapEntry is the primitive used
// /// to track claimed *PriorityMutex.
// pub type MutexMapEntry<T> = Arc<PriorityMutex<T>>;

// struct MutexMapGuard<'a, T> {
//     data: PriorityMutexGuard<'a, T>,
//     mutex: &'a MutexMap<T>,
// }

// impl<'a, T> Deref for MutexMapGuard<'a, T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }

// impl<'a, T> DerefMut for MutexMapGuard<'a, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.data
//     }
// }

// impl<'a, T> Drop for MutexMapGuard<'a, T> {
//     fn drop(&mut self) {
//         self.mutex.unlock()
//     }
// }

// /// `MutexMap` is a struct that allows for
// /// acquiring a `PriorityMutex` via a string identifier
// /// or for acquiring a global mutex that blocks
// /// the acquisition of any identifier mutexes.
// ///
// /// This is useful for coordinating concurrent, non-overlapping
// /// writes in the storage package.
// pub struct MutexMap<T> {
//     entries: RwLock<ShardedMap<MutexMapEntry<T>>>,
// }

// impl<T> MutexMap<T> {
//     // returns a new `MutexMap`.
//     pub fn new(shards: usize) -> MutexMap<T> {
//         Self {
//             entries: RwLock::new(ShardedMap::new(shards)),
//         }
//     }

//     // acquires an exclusive lock across an entire `MutexMap`.
//     pub fn global_lock(&self) -> RwLockWriteGuard<ShardedMap<MutexMapEntry<T>>> {
//         self.entries.write()
//     }

//     /// acquires a lock for a particular identifier, as long
//     /// as no other caller has the global mutex or a lock
//     /// by the same identifier.
//     pub fn lock(&self, identifier: &str, priority: bool) -> MutexMapGuard<T>
//     where
//         T: Default,
//     {
//         // We acquire a RLock on m.globalMutex before
//         // acquiring our identifier lock to ensure no
//         // thread holds an identifier mutex while
//         // the m.globalMutex is also held.
//         let entries = self.entries.read();

//         // We acquire m when adding items to m.table
//         // so that we don't accidentally overwrite
//         // lock created by another thread.
//         let mut data = entries.lock(identifier, priority);
//         if !data.contains_key(identifier) {
//             data.insert(identifier.into(), Default::default());
//         }

//         let entry = data.get(identifier).unwrap().clone();
//         let entry_value = entry.lock(priority);

//         // Once we have a RLock, it is safe to acquire an identifier lock.
//         MutexMapGuard {
//             data: entry_value,
//             mutex: self,
//         }
//     }

//     // Unlock releases a lock held for a particular identifier.
//     fn unlock(&self) {
//         todo!()
//     }
// }
