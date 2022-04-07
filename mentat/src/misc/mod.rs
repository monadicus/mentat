//! This module defines misc structs for Mentat.

use serde::{Deserialize, Serialize};
use serde_json::Value;

mod operation_status;
pub use operation_status::*;

mod peer;
pub use peer::*;

mod sync_status;
pub use sync_status::*;

mod version;
pub use version::*;
