use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use crossbeam_channel::{bounded, Sender};
use parking_lot::{Mutex, MutexGuard};

/// An implementation of a "scoped lock" of a mutex. When this structure is dropped (falls out of scope), the lock will be unlocked.
/// The data protected by the mutex can be accessed through this guard via its Deref and DerefMut implementations.
pub struct PriorityMutexGuard<'a, T> {
    data: MutexGuard<'a, T>,
    mutex: &'a PriorityMutex<T>,
}

impl<'a, T> Deref for PriorityMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> DerefMut for PriorityMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> Drop for PriorityMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}

/// PriorityMutex is a special type of mutex
/// that allows callers to request priority
/// over other callers. This can be useful
/// if there is a "hot path" in an application
/// that requires lock access.
///
/// WARNING: It is possible to cause lock starvation
/// if not careful (i.e. only high priority callers
/// ever do work).
pub struct PriorityMutex<T> {
    pub(crate) data: Mutex<T>,
    pub(crate) high: Mutex<VecDeque<Sender<()>>>,
    pub(crate) low: Mutex<VecDeque<Sender<()>>>,
}

impl<T> PriorityMutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    pub fn new(v: T) -> Self {
        Self {
            data: Mutex::new(v),
            high: Default::default(),
            low: Default::default(),
        }
    }

    /// Lock attempts to acquire either a high or low
    /// priority mutex. When priority is true, a lock
    /// will be granted before other low priority callers.
    pub fn lock(&self, priority: bool) -> PriorityMutexGuard<T> {
        let data = self.data.try_lock().unwrap_or_else(|| {
            let (s, r) = bounded(0);
            if priority {
                self.high.lock().push_back(s);
            } else {
                self.low.lock().push_back(s);
            }
            r.recv().unwrap_err();
            self.data.lock()
        });
        PriorityMutexGuard { data, mutex: self }
    }

    /// Unlock selects the next highest priority lock
    /// to grant. If there are no locks to grant, it
    /// sets the value of m.lock to false.
    fn unlock(&self) {
        self.high
            .lock()
            .pop_front()
            .is_none()
            .then(|| self.low.lock().pop_front());
    }
}

impl<T: Default> Default for PriorityMutex<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            high: Default::default(),
            low: Default::default(),
        }
    }
}
