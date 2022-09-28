use serde_json::Value;

pub fn set_json(json: &mut Value, path: &str, value: Value) -> Result<(), String> {
    let mut inner_value = json;
    let keys = path.split('.').collect::<Vec<_>>();
    for key in &keys[0..keys.len() - 1] {
        inner_value = inner_value
            .get_mut(key)
            .ok_or(format!("{key} of path {path} is not present in json"))?;
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
