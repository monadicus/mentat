//! Defines struct models for Rosetta API.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{errors::MentatError, identifiers::*, misc::*};

mod allow;
pub use allow::*;

mod amount;
pub use amount::*;

mod balance_exemption;
pub use balance_exemption::*;

mod block_event_type;
pub use block_event_type::*;

mod block_event;
pub use block_event::*;

mod block_transaction;
pub use block_transaction::*;

mod block;
pub use block::*;

mod coin_action;
pub use coin_action::*;

mod coin_change;
pub use coin_change::*;

mod coin;
pub use coin::*;

mod currency;
pub use currency::*;

mod curve_type;
pub use curve_type::*;

mod direction;
pub use direction::*;

mod exemption_type;
pub use exemption_type::*;

mod operation;
pub use operation::*;

mod operator;
pub use operator::*;

mod public_key;
pub use public_key::*;

mod related_transaction;
pub use related_transaction::*;

mod signature_type;
pub use signature_type::*;

mod signature;
pub use signature::*;

mod signing_payload;
pub use signing_payload::*;

mod transaction;
pub use transaction::*;

/// A trait that sorts the various aspects
/// of the types in the crate for testing purposes.
pub(crate) trait Sortable {
    /// Takes a reference to the object.
    /// Then clones it and sorts the fields.
    fn sort(&self) -> Self;
}
