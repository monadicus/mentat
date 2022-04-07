#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![doc = include_str!("../../README.md")]

pub mod errors;
pub mod identifiers;
pub mod misc;
pub mod models;
pub mod requests;
pub mod responses;

#[cfg(feature = "sdk")]
pub mod keys;

#[allow(clippy::all, clippy::missing_docs_in_private_items)]
#[cfg(feature = "server")]
#[path = ""]
mod server_reexport {
    pub mod api;
    pub mod cache;
    pub mod conf;
    pub mod server;

    pub use axum::{self, async_trait, Json};
    pub use indexmap::IndexMap;
    pub use mentat_macros::{main, mentat};
    pub use reqwest::{IntoUrl, Url};
    pub use serde;
    pub use serde_json;
    pub use tokio;
    pub use tracing;

    pub mod macro_exports {
        pub use axum::{
            extract::{self, ConnectInfo, Extension, Json},
            routing,
            Router,
        };
        pub use tracing::Instrument;

        pub use super::{
            api::*,
            cache::Cache,
            conf::Configuration,
            server::{RpcCaller, Server},
            *,
        };
        pub use crate::requests::*;
    }
}

#[cfg(feature = "server")]
pub use server_reexport::*;

#[cfg(feature = "client")]
pub mod client;
