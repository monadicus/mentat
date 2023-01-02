//! The server module for building a rosetta spec server.

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

pub mod api;
pub mod conf;
pub mod server;

pub use axum;
pub use indexmap;
//
/// A macro for generating mentat routes from a user supplied `main`
/// function.
///
/// The function should return a [`server::Server`] instance with a custom
/// [`server::ServerType`]. If a [`cache::CacheInner`] implementation is
/// supplied then it will use it to perform request caching, otherwise no
/// caching will be performed.
///
/// If the `main` function is only calling
/// `Server::default()` then consider using the [`macro@mentat`] macro
/// instead.
pub use mentat_macros::main;
/// A macro for generating mentat routes from a default [`server::Server`]
/// instance.
///
/// When this macro is used it will generate its own main function, so the
/// user doesn't need to include one. If a [`cache::CacheInner`]
/// implementation is supplied then it will use it to perform request
/// caching, otherwise no caching will be performed.
///
/// If you prefer to use your own `main` function, consider using the
/// [`macro@main`] macro instead.
pub use mentat_macros::mentat;
pub use reqwest;
pub use serde;
pub use serde_json;
pub use sysinfo;
pub use tokio;
pub use tracing;
pub use tracing_subscriber;
pub use tracing_tree;

#[doc(hidden)]
pub mod macro_exports {
    pub use std::sync::Arc;

    pub use axum::{
        extract::{self, ConnectInfo, Extension, Json, State},
        routing,
        Router,
    };
    pub use mentat_types::*;
    pub use tracing::Instrument;

    pub use super::{
        api::*,
        conf::{Configuration, ServerPid},
        server::Server,
        *,
    };
}
