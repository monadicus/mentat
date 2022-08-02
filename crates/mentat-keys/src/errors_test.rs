use std::collections::HashMap;

use create::KeysError;

#[test]
fn test_err() {
    let mut tests = HashMap::new();
    tests.insert(
        "is a keys error",
        (Err(KeysError::ErrPrivKeyLengthInvalid), true),
    );
    tests.insert("not a keys error", (Err("blah"), false));

    for (name, test) in tests.iter() {
        let is = matches!(
            std::mem::discriminant(&KeysError),
            std::mem::discriminant(test.0.unwrap_err())
        );
        assert_eq!(test.1, is);
    }
}
