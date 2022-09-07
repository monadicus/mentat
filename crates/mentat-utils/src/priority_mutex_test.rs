use std::{
    sync::Arc,
    thread::{sleep, spawn},
    time::Duration,
};

use crate::priority_mutex::PriorityMutex;

#[test]
fn test_priority_mutex() {
    let mut expected = vec![false; 60];
    expected[0..10].iter_mut().for_each(|e| *e = true);

    let arr = Arc::new(PriorityMutex::new(Vec::new()));

    let mut handles = Vec::new();

    // Lock while adding all locks
    let arr_lock = arr.lock(true);

    // Add a bunch of low prio items
    for i in &mut expected[10..60] {
        *i = false;
        let arr = arr.clone();
        handles.push(spawn(move || {
            arr.lock(false).push(false);
        }))
    }

    // Add a few high prio items
    for i in &mut expected[0..10] {
        *i = true;
        let arr = arr.clone();
        handles.push(spawn(move || {
            arr.lock(true).push(true);
        }))
    }

    // Wait for all threads to ask for lock
    sleep(Duration::from_secs(1));

    // Ensure number of expected locks is correct
    assert_eq!(arr.high.lock().len(), 10);
    assert_eq!(arr.low.lock().len(), 50);

    // drop lock so threads can run
    drop(arr_lock);
    for h in handles {
        h.join().unwrap();
    }

    // Ensure lock is no longer occupied
    assert!(!arr.data.is_locked());

    // Check results array to ensure all of the high priority items processed first,
    // followed by all of the low priority items.
    assert_eq!(expected, *arr.lock(true));
}
