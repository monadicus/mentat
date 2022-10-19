use serde_json::Value;

pub fn set_json(json: &mut Value, path: &str, value: Value) -> Result<(), String> {
    let mut inner_value = json;
    let keys = path.split('.').collect::<Vec<_>>();
    for key in &keys[0..keys.len() - 1] {
        // using `if let` here with `get_mut` leads to a double borrow error in the else branch. so we do two get operations instead
        if inner_value.get(key).is_none() {
            inner_value
                .as_object_mut()
                .ok_or(format!("can't set {path}: field not an object"))?
                .insert(key.to_string(), Value::Object(Default::default()));
        }
        inner_value = inner_value.get_mut(key).unwrap();
    }

    let object = inner_value
        .as_object_mut()
        .ok_or(format!("can't set {path}: field not an object"))?;
    object.insert(keys.last().unwrap().to_string(), value);

    Ok(())
}

pub fn get_json<'a>(json: &'a Value, path: &str) -> Option<&'a Value> {
    let mut inner_value = json;
    let keys = path.split('.').collect::<Vec<_>>();
    for key in keys {
        inner_value = inner_value.get(key)?;
    }
    Some(inner_value)
}
