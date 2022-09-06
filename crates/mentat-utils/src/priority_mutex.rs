use std::sync::atomic::AtomicBool;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{Mutex, MutexGuard};

/// PriorityMutex is a special type of mutex
/// that allows callers to request priority
/// over other callers. This can be useful
/// if there is a "hot path" in an application
/// that requires lock access.
///
/// WARNING: It is possible to cause lock starvation
/// if not careful (i.e. only high priority callers
/// ever do work).

// todo: wrong
struct PriorityChannel<T> {
    sender: Sender<()>,
    receiver: Receiver<T>,
}

pub struct PriorityMutex<T> {
    high: Vec<PriorityChannel<T>>,
    low: Vec<PriorityChannel<T>>,
    mutex_lock: AtomicBool,
}
