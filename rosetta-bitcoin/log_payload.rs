#[cfg(debug_assertions)]
use std::{fmt, fs, io::Write};

#[cfg(debug_assertions)]
pub fn log_payload<T: fmt::Display>(route: &str, payload: T) {
    let t = format!("{}: {}\n", route, payload);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.json")
        .unwrap();
    file.write_all(t.as_bytes()).unwrap();
}
