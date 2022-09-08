use std::{sync::Arc, thread::spawn};

use crossbeam_channel::bounded;

use crate::sharded_map::ShardedMap;

#[test]
fn test_sharded_map() {
    let map = Arc::new(ShardedMap::<String>::new(2));
    let mut handles = Vec::new();

    // To test locking, we use channels
    // that will cause deadlock if not executed
    // concurrently.
    let (a_s, a_r) = bounded::<()>(0);
    let (b_s, b_r) = bounded::<()>(0);

    let tmp_map = map.clone();
    handles.push(spawn(move || {
        let mut s = tmp_map.lock("a", false);
        assert_eq!(s.len(), 0);
        s.insert("test".into(), "a".into());
        a_r.recv().unwrap_err();
        drop(b_s);
        drop(s);
    }));

    let tmp_map = map.clone();
    handles.push(spawn(move || {
        let mut s = tmp_map.lock("b", false);
        assert_eq!(s.len(), 0);
        s.insert("test".into(), "b".into());
        drop(a_s);
        b_r.recv().unwrap_err();
        drop(s);
    }));

    for h in handles {
        h.join().unwrap()
    }

    // Ensure keys set correctly
    let s = map.lock("a", false);
    assert_eq!(s.len(), 1);
    assert_eq!(s.get("test").unwrap(), "a");

    let s = map.lock("b", false);
    assert_eq!(s.len(), 1);
    assert_eq!(s.get("test").unwrap(), "b");
}
