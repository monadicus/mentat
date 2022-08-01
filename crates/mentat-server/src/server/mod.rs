//! Defines the `Server` methods and launcher for Mentat.

use serde::de::DeserializeOwned;
use serde_json::Value;
use sysinfo::{Pid, PidExt};
use tracing_subscriber::util::SubscriberInitExt;
mod middleware_checks;
mod rpc_caller;

use std::net::SocketAddr;

use axum::{extract::Extension, handler::Handler, http::Extensions, middleware, Router};
use mentat_types::{MentatError, Result};
pub use rpc_caller::*;
use tracing::{info, Dispatch};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

use self::middleware_checks::{middleware_checks, NetworkIdentifierCheck};
use crate::{api::*, conf::*};

/// Contains the types required to construct a mentat [`Server`].
///
/// Can be initiated with the [`super::main`] macro to construct a custom
/// instance of [`Server`] using [`ServerBuilder`], or with the
/// [`super::mentat`] macro if a default instance using your custom types is
/// preferred.
///
/// ```no_run
/// struct MentatBitcoin;
/// impl ServerType for MentatBitcoin {
///     type CallApi = call_api::BitcoinCallApi;
///     type ConstructionApi = construction_api::BitcoinConstructionApi;
///     type DataApi = data_api::BitcoinDataApi;
///     type IndexerApi = indexer_api::BitcoinIndexerApi;
///     type CustomConfig = node::NodeConfig;
/// }
/// ```
pub trait ServerType: Sized + 'static {
    /// The blockchain's `CallApi` Rosetta implementation.
    type CallApi: CallerCallApi;
    /// The blockchain's `ConstructionApi` Rosetta implementation.
    type ConstructionApi: CallerConstructionApi;
    /// The blockchain's `CallerDataApi` Rosetta implementation.
    type DataApi: CallerDataApi;
    /// The blockchain's `IndexerApi` Rosetta implementation.
    type IndexerApi: CallerIndexerApi;
    /// Any optional endpoints for the Mentat implementation.
    type OptionalApi: OptionalApi;
    /// The nodes's `NodeConf` implementation.
    type CustomConfig: DeserializeOwned + NodeConf;

    /// A function to implement middleware checks.
    fn middleware_checks(extensions: &Extensions, json: &Value) -> Result<()> {
        NetworkIdentifierCheck::check::<Self>(extensions, json)
    }

    /// Sets up a tracing subscriber dispatch
    fn setup_logging() -> Dispatch {
        let tracer = opentelemetry_jaeger::new_pipeline()
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap_or_else(|e| panic!("Failed to start opentelemtry_jaeger: `{e}`"));
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        Registry::default()
            .with(EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "debug,tower_http=debug".to_string()),
            ))
            .with(
                HierarchicalLayer::new(2)
                    .with_targets(true)
                    .with_bracketed_fields(true),
            )
            .with(tracing_error::ErrorLayer::default())
            .with(telemetry)
            .into()
    }

    /// Shuts down any necessary logging details for Mentat.
    fn teardown_logging() {
        opentelemetry::global::shutdown_tracer_provider();
    }
}

/// The Struct for building a Server.
pub struct ServerBuilder<Types: ServerType> {
    /// The optional API endpoints.
    optional_api: Option<Types::OptionalApi>,
    /// The call API endpoints.
    call_api: Option<Types::CallApi>,
    /// The construction API endpoints.
    construction_api: Option<Types::ConstructionApi>,
    /// The data API endpoints.
    data_api: Option<Types::DataApi>,
    /// The indexer API endpoints.
    indexer_api: Option<Types::IndexerApi>,
    /// The optional configuration details.
    configuration: Option<Configuration<Types::CustomConfig>>,
}

impl<Types: ServerType> Default for ServerBuilder<Types> {
    fn default() -> Self {
        Self {
            optional_api: None,
            call_api: None,
            configuration: None,
            construction_api: None,
            data_api: None,
            indexer_api: None,
        }
    }
}

impl<Types: ServerType> ServerBuilder<Types> {
    /// Builds the Server.
    pub fn build(self) -> Server<Types> {
        Server {
            optional_api: self
                .optional_api
                .expect("You did not set the additional api."),
            call_api: self.call_api.expect("You did not set the call api."),
            configuration: self
                .configuration
                .expect("You did not set the custom configuration."),
            construction_api: self
                .construction_api
                .expect("You did not set the construction api."),
            data_api: self.data_api.expect("You did not set the data api."),
            indexer_api: self.indexer_api.expect("You did not set the indexer api."),
        }
    }

    /// Sets the optional API on the builder.
    pub fn optional_api(mut self, a: Types::OptionalApi) -> Self {
        self.optional_api = Some(a);
        self
    }

    /// Sets the call API on the builder.
    pub fn call_api(mut self, a: Types::CallApi) -> Self {
        self.call_api = Some(a);
        self
    }

    /// Sets the custom configuration from a cli arg on the builder.
    pub fn custom_configuration_from_arg(self) -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("Expected usage: <{}> <configuration file>", args[0]);
            std::process::exit(1);
        }

        let path = std::path::PathBuf::from(&args[1]);
        self.custom_configuration(&path)
    }

    /// Sets the custom configuration on the builder from a path.
    pub fn custom_configuration(mut self, path: &std::path::Path) -> Self {
        self.configuration = Some(Configuration::load(path));
        self
    }

    /// Sets the construction API on the builder.
    pub fn construction_api(mut self, a: Types::ConstructionApi) -> Self {
        self.construction_api = Some(a);
        self
    }

    /// Sets the data API on the builder.
    pub fn data_api(mut self, a: Types::DataApi) -> Self {
        self.data_api = Some(a);
        self
    }

    /// Sets the indexer API on the builder.
    pub fn indexer_api(mut self, a: Types::IndexerApi) -> Self {
        self.indexer_api = Some(a);
        self
    }
}

/// The server struct for running the Rosetta server.
pub struct Server<Types: ServerType> {
    /// The optional API endpoints.
    pub optional_api: Types::OptionalApi,
    /// The call API endpoints.
    pub call_api: Types::CallApi,
    /// The configuration API endpoints.
    pub configuration: Configuration<Types::CustomConfig>,
    /// The construction API endpoints.
    pub construction_api: Types::ConstructionApi,
    /// The data API endpoints.
    pub data_api: Types::DataApi,
    /// The indexer API endpoints.
    pub indexer_api: Types::IndexerApi,
}

impl<Types: ServerType> Default for Server<Types> {
    fn default() -> Self {
        Self {
            optional_api: Default::default(),
            call_api: Default::default(),
            configuration: Types::CustomConfig::load_config(),
            construction_api: Default::default(),
            data_api: Default::default(),
            indexer_api: Default::default(),
        }
    }
}

impl<Types: ServerType> Server<Types> {
    /// WARNING: Do not use this method outside of Mentat! Use the `mentat` or
    /// `main` macros instead
    #[doc(hidden)]
    pub async fn serve(self, mut app: Router) {
        color_backtrace::install();
        Types::setup_logging().init();

        let node_pid = Types::CustomConfig::start_node(&self.configuration);
        let server_pid = Pid::from_u32(std::process::id());

        let rpc_caller = RpcCaller::new(&self.configuration);
        let addr = SocketAddr::from((self.configuration.address, self.configuration.port));

        app = app
            .route_layer(middleware::from_fn(middleware_checks::<Types>))
            .layer(
                tower::ServiceBuilder::new()
                    .layer(Extension(self.configuration))
                    .layer(Extension(node_pid))
                    .layer(Extension(server_pid))
                    .layer(Extension(rpc_caller)),
            )
            .fallback(MentatError::not_found.into_service());

        info!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap_or_else(|err| panic!("Failed to listen on addr `{addr}`: `{err}`."));

        Types::teardown_logging();
    }
}
