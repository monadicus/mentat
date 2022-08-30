//! Defines a configuration file for the Mentat server.

use serde::{Deserialize, Serialize};

mod configuration;
pub use configuration::*;

mod mode;
pub use mode::*;

mod network;
pub use network::*;

mod asserter;
pub use asserter::*;
