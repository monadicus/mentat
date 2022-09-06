use crate::bst::Bst;

macro_rules! assert_none {
    ($v:expr) => {
        if let Some(v) = $v {
            panic!("expected None, found {v:?}")
        }
    };
}

#[test]
fn test_bts() {
    let mut bst = Bst::default();

    // Test empty BST
    assert_none!(bst.get(1));
    assert_none!(bst.min());
    assert!(bst.is_empty());

    // Set 1 key and ensure it is the min
    bst.set(1, 10);
    assert!(!bst.is_empty());
    assert_eq!(10, bst.get(1).unwrap().value);
    assert_eq!(1, bst.min().unwrap().key);
    assert_none!(bst.get(10));

    // Overwrite existing key
    bst.set(1, 11);
    assert_eq!(11, bst.get(1).unwrap().value);
    assert_eq!(1, bst.min().unwrap().key);

    // Add a key that will be put in "left"
    // of root
    bst.set(0, 11);
    assert_eq!(11, bst.get(0).unwrap().value);
    assert_eq!(0, bst.min().unwrap().key);

    // Delete root
    bst.delete(1);
    assert_eq!(11, bst.get(0).unwrap().value);
    assert_eq!(0, bst.min().unwrap().key);
    assert_none!(bst.get(1));

    // Add keys to the "right" of new root
    bst.set(3, 33);
    bst.set(2, 22);

    // Delete already deleted item
    bst.delete(1);
    assert!(!bst.is_empty());

    // Delete root again
    bst.delete(0);
    bst.delete(0);
    assert!(!bst.is_empty());

    // Ensure 2 is the min key after root
    // deleted
    assert_eq!(2, bst.min().unwrap().key);

    // Delete all items
    bst.delete(3);
    bst.delete(2);
    assert!(bst.is_empty());
    assert_none!(bst.min());
}
