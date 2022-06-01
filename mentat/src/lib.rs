#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]
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
mod server_exports {
    pub mod api;
    pub mod cache;
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
    ///
    /// ``` no_run
    /// #[derive(Clone)]
    /// struct MentatSnarkos;
    ///
    /// impl ServerType for MentatSnarkos {
    ///     type CallApi = call_api::SnarkosCallApi;
    ///     type ConstructionApi = construction_api::SnarkosConstructionApi;
    ///     type CustomConfig = node::NodeConfig;
    ///     type DataApi = data_api::SnarkosDataApi;
    ///     type IndexerApi = indexer_api::SnarkosIndexerApi;
    /// }
    ///
    /// #[mentat::main(DefaultCacheInner)]
    /// async fn main() -> Server<MentatSnarkos> {
    ///     println!("hello rosetta!");
    ///     Server::default()
    /// }
    /// ```
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
    ///
    /// ```no_run
    /// #[mentat(DefaultCacheInner)]
    /// struct MentatBitcoin;
    ///
    /// impl ServerType for MentatBitcoin {
    ///     type CallApi = call_api::BitcoinCallApi;
    ///     type ConstructionApi = construction_api::BitcoinConstructionApi;
    ///     type CustomConfig = node::NodeConfig;
    ///     type DataApi = data_api::BitcoinDataApi;
    ///     type IndexerApi = indexer_api::BitcoinIndexerApi;
    /// }
    /// ```
    pub use mentat_macros::mentat;
    pub use reqwest;
    pub use serde;
    pub use serde_json;
    pub use sysinfo;
    pub use tokio;
    pub use tracing;

    #[doc(hidden)]
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
            server::{RpcCaller, RpcResponse, Server},
            *,
        };
        pub use crate::requests::*;
    }
}

#[cfg(feature = "server")]
pub use server_exports::*;

#[allow(clippy::all, clippy::missing_docs_in_private_items)]
#[cfg(feature = "client")]
#[path = ""]
mod client_rexport {
    pub mod client;
    pub use serde;
    pub use serde_json;
}

#[cfg(feature = "client")]
pub use client_rexport::*;
