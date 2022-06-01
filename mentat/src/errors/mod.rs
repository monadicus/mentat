//! Defines errors for the Rosetta API.

#[cfg(feature = "server")]
mod server_error;
#[cfg(feature = "server")]
pub use server_error::*;
