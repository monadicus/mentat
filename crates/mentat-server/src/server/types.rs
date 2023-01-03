//! Defines `ServerType` which defines all types for a `Server`.

use std::sync::Arc;

use axum::Router;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

use super::state::AppState;
use crate::{api::*, conf::*};

/// Contains the types required to construct a mentat [`Server`].
///
/// Can be initiated with the [`super::main`] macro to construct a custom
/// instance of [`Server`] using [`ServerBuilder`], or with the
/// [`super::mentat`] macro if a default instance using your custom types is
/// preferred.
pub trait ServerType: Sized + 'static {
    /// The blockchain's `AccountApi` Rosetta implementation.
    type AccountApi: AccountApi<NodeCaller = Self::NodeCaller>;
    /// The blockchain's `BlockApi` Rosetta implementation.
    type BlockApi: BlockApi<NodeCaller = Self::NodeCaller>;
    /// The blockchain's `CallApi` Rosetta implementation.
    type CallApi: CallApi<NodeCaller = Self::NodeCaller>;
    /// The blockchain's `ConstructionApi` Rosetta implementation.
    type ConstructionApi: ConstructionApi<NodeCaller = Self::NodeCaller>;
    /// The blockchain's `Events` Rosetta implementation.
    type EventsApi: EventsApi<NodeCaller = Self::NodeCaller>;
    /// The blockchain's `MempoolApi` Rosetta implementation.
    type MempoolsApi: MempoolApi<NodeCaller = Self::NodeCaller>;
    /// The blockchain's `NetworkApi` Rosetta implementation.
    type NetworkApi: NetworkApi<NodeCaller = Self::NodeCaller>;
    /// Any optional endpoints for the Mentat implementation.
    type OptionalApi: OptionalApi<NodeCaller = Self::NodeCaller> + Send + Sync;
    /// The blockchain's `SearchApi` Rosetta implementation.
    type SearchApi: SearchApi<NodeCaller = Self::NodeCaller>;
    /// The Caller used to interact with the node.
    type NodeCaller: From<Configuration<Self::CustomConfig>> + Send + Sync + Clone + std::fmt::Debug;
    /// The nodes's `NodeConf` implementation.
    type CustomConfig: serde::de::DeserializeOwned + NodeConf;

    /// returns the asserter to be used when asserting requests
    fn init_asserters(_config: &Configuration<Self::CustomConfig>) -> AsserterTable;

    /// an optional function to add middleware to the axum server. by default
    /// this does nothing. look at the provided functions in [`middleware`]
    /// for help with constructing middleware layers
    fn middleware(
        _config: &Configuration<Self::CustomConfig>,
        router: Router<Arc<AppState<Self::CustomConfig>>>,
    ) -> Router<Arc<AppState<Self::CustomConfig>>> {
        router
    }

    /// Sets up a tracing subscriber dispatch
    fn setup_logging() {
        let collector_port =
            std::env::var("MENTANT_COLLECTOR_PORT").unwrap_or_else(|_| "14268".to_string());
        // TODO test if still needed
        // let agent_port = std::env::var("MENTANT_AGENT_PORT").unwrap_or_else(|_|
        // "6831".to_string());

        opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
        let tracer = opentelemetry_jaeger::new_collector_pipeline()
            .with_hyper()
            .with_endpoint(format!("http://localhost:{collector_port}/api/traces"))
            // .with_endpoint(format!("0.0.0.0:{agent_port}"))
            .with_service_name(env!("CARGO_PKG_NAME"))
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap_or_else(|e| panic!("Failed to start opentelemtry_jaeger: `{e}`"));

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        let registry = tracing_subscriber::Registry::default()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "debug,tower_http=debug".to_string()),
            ))
            .with(
                tracing_tree::HierarchicalLayer::new(2)
                    .with_targets(true)
                    .with_bracketed_fields(true),
            )
            .with(tracing_error::ErrorLayer::default())
            .with(telemetry)
            .into();
        tracing::dispatcher::set_global_default(registry)
            .unwrap_or_else(|err| panic!("Failed to set logger dispatcher: `{err}`"));
    }

    /// Shuts down any necessary logging details for Mentat.
    fn teardown_logging() {
        opentelemetry::global::shutdown_tracer_provider();
    }
}
