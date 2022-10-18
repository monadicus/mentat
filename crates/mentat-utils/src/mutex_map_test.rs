use std::{
    ops::Deref,
    sync::Arc,
    thread::{sleep, spawn},
    time::Duration,
};

use crossbeam_channel::unbounded;
use parking_lot::Mutex;

use crate::{mutex_map::MutexMap, sharded_map::DEFAULT_SHARDS};

#[test]
fn test_mutex_map() {
    let mut threads = Vec::new();
    let arr = Arc::new(Mutex::new(Vec::new()));
    let m = Arc::new(MutexMap::<Arc<()>>::new(DEFAULT_SHARDS));

    // Lock while adding all locks
    let lock = m.global_lock();

    // To test locking, we use channels
    // that will cause deadlock if not executed
    // concurrently.
    let (a_tx, a_rx) = unbounded::<()>();
    let (b_tx, b_rx) = unbounded::<()>();

    {
        let m = m.clone();
        let arr = arr.clone();
        threads.push(spawn(move || {
            m.inspect("a", false, |x| {
                assert_eq!(Arc::strong_count(x), 1);
                a_rx.recv().unwrap_err();
                arr.lock().push("a");
                drop(b_tx);
            });
        }));
    }

    {
        let m = m.clone();
        let arr = arr.clone();
        threads.push(spawn(move || {
            m.inspect("b", false, |x| {
                assert_eq!(Arc::strong_count(x), 1);
                drop(a_tx);
                b_rx.recv().unwrap_err();
                arr.lock().push("b");
            });
        }));
    }

    // NOTE: BEHAVIORAL DIFFERENCES BETWEEN GO AND RUST
    // parking_lot uses a "fair" model for RWLocks where readers will block when a writer is trying to acquire a lock even if the read lock is free and the read lock was queued before the write lock.
    // the built in RwLock makes no guarantee of what model is used on what system, but on my machine seemed to match the "fair" model in use by parking_lot
    // meanwhile go uses a greedy model, where readers will always be given priority over writers.
    // this means that, to match the output expectations for this test, i needed to move the order in which the threads get executed so that `global-b` attempts to acquire a lock last. i also added a sleep statement just to ensure the read locks are queued first
    sleep(Duration::from_secs(1));
    // Add another GLock
    {
        let m = m.clone();
        let arr = arr.clone();
        threads.push(spawn(move || {
            let lock = m.global_lock();
            arr.lock().push("global-b");
            drop(lock);
        }));
    }

    sleep(Duration::from_secs(1));

    // Ensure number of expected locks is correct
    let total_keys = lock.lock("a", true).len() + lock.lock("b", true).len();
    assert_eq!(total_keys, 0);
    arr.lock().push("global-a");
    drop(lock);
    threads.into_iter().for_each(|t| t.join().unwrap());

    // Check results array to ensure all of the high priority items processed first,
    // followed by all of the low priority items.
    assert_eq!(
        arr.lock().deref(),
        &[
            "global-a",
            "a",
            "b",
            "global-b", // must wait until all other locks complete
        ]
    );

    // Ensure lock is no longer occupied
    let lock = m.global_lock();
    let total_keys = lock.lock("a", true).len() + lock.lock("b", true).len();
    assert_eq!(total_keys, 0);
}
