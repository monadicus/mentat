//! Defines struct identifiers for Rosetta API.

use serde::{Deserialize, Serialize};

use super::*;

mod account_identifier;
pub use account_identifier::*;

mod block_identifier;
pub use block_identifier::*;

mod coin_identifier;
pub use coin_identifier::*;

mod network_identifier;
pub use network_identifier::*;

mod operation_identifier;
pub use operation_identifier::*;

mod partial_block_identifier;
pub use partial_block_identifier::*;

mod sub_account_identifier;
pub use sub_account_identifier::*;

mod sub_network_identifier;
pub use sub_network_identifier::*;

mod transaction_identifier;
pub use transaction_identifier::*;
