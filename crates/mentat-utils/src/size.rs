const BYTES_IN_KB: usize = 1024;

// converts Byes to MegaBytes.
pub fn b_to_mb(b: f64) -> f64 {
    b / BYTES_IN_KB as f64 / BYTES_IN_KB as f64
}

// TODO: EstimatedSize trait should be in here. but this crate imports types, and types imports this trait. so it would cause a cycling dependency. just keeping it in types for now
