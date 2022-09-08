//! Defines misc structs for Mentat.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::*;

mod operation_status;
pub use operation_status::*;

mod peer;
pub use peer::*;

mod sync_status;
pub use sync_status::*;

mod version;
pub use version::*;

pub type Metadata = IndexMap<String, Value>;
