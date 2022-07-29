//! a struct for the /optional/synced endpoint that contains local and global
//! tip

use serde::{Deserialize, Serialize};

/// contains local and global chain tips
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Synced {
    pub local_tip: usize,
    pub global_tip: usize,
}
