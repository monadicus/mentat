//! Defines the `Server` methods and launcher for Mentat.

use serde::de::DeserializeOwned;
use sysinfo::{Pid, PidExt};
pub mod middleware;

use std::net::SocketAddr;

use axum::{extract::Extension, handler::Handler, Router};
use mentat_types::MentatError;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

use crate::{api::*, conf::*, server::middleware::content_type_middleware};

/// Contains the types required to construct a mentat [`Server`].
///
/// Can be initiated with the [`super::main`] macro to construct a custom
/// instance of [`Server`] using [`ServerBuilder`], or with the
/// [`super::mentat`] macro if a default instance using your custom types is
/// preferred.
pub trait ServerType: Sized + 'static {
    /// The blockchain's `AccountApi` Rosetta implementation.
    type AccountApi: AccountApiRouter;
    /// The blockchain's `BlockApi` Rosetta implementation.
    type BlockApi: BlockApiRouter;
    /// The blockchain's `CallApi` Rosetta implementation.
    type CallApi: CallApiRouter;
    /// The blockchain's `ConstructionApi` Rosetta implementation.
    type ConstructionApi: ConstructionApiRouter;
    /// The blockchain's `Events` Rosetta implementation.
    type EventsApi: EventsApiRouter;
    /// The blockchain's `MempoolApi` Rosetta implementation.
    type MempoolsApi: MempoolApiRouter;
    /// The blockchain's `NetworkApi` Rosetta implementation.
    type NetworkApi: NetworkApiRouter;
    /// Any optional endpoints for the Mentat implementation.
    type OptionalApi: OptionalApiRouter;
    /// The blockchain's `SearchApi` Rosetta implementation.
    type SearchApi: SearchApiRouter;
    /// The Caller used to interact with the node.
    type NodeCaller: From<Configuration<Self::CustomConfig>> + Send + Sync + Clone;
    /// The nodes's `NodeConf` implementation.
    type CustomConfig: DeserializeOwned + NodeConf;

    /// returns the asserter to be used when asserting requests
    fn init_asserters(_config: &Configuration<Self::CustomConfig>) -> AsserterTable;

    /// an optional function to add middleware to the axum server. by default
    /// this does nothing. look at the provided functions in [`middleware`]
    /// for help with constructing middleware layers
    fn middleware(_config: &Configuration<Self::CustomConfig>, _router: &mut Router) {}

    /// Sets up a tracing subscriber dispatch
    fn setup_logging() -> tracing::Dispatch {
        let collector_port =
            std::env::var("MENTANT_COLLECTOR_PORT").unwrap_or_else(|_| "14268".to_string());
        let agent_port = std::env::var("MENTANT_AGENT_PORT").unwrap_or_else(|_| "6831".to_string());

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_collector_endpoint(format!("http://localhost:{collector_port}/api/traces"))
            .with_agent_endpoint(format!("0.0.0.0:{agent_port}"))
            .with_service_name(env!("CARGO_PKG_NAME"))
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
    /// The Account API endpoints.
    account_api: Option<Types::AccountApi>,
    /// The Block API endpoints.
    block_api: Option<Types::BlockApi>,
    /// The Call API endpoints.
    call_api: Option<Types::CallApi>,
    /// The Construction API endpoints.
    construction_api: Option<Types::ConstructionApi>,
    /// The Events API endpoints.
    events_api: Option<Types::EventsApi>,
    /// The Mempool API endpoints.
    mempool_api: Option<Types::MempoolsApi>,
    /// The network API endpoints.
    network_api: Option<Types::NetworkApi>,
    /// The search API endpoints.
    search_api: Option<Types::SearchApi>,
    /// The Optional API endpoints.
    optional_api: Option<Types::OptionalApi>,
    /// The Caller used to interact with the node
    node_caller: Option<Types::NodeCaller>,
    /// The optional configuration details.
    configuration: Option<Configuration<Types::CustomConfig>>,
}

impl<Types: ServerType> Default for ServerBuilder<Types> {
    fn default() -> Self {
        Self {
            account_api: None,
            block_api: None,
            call_api: None,
            construction_api: None,
            events_api: None,
            mempool_api: None,
            network_api: None,
            search_api: None,
            optional_api: None,
            node_caller: None,
            configuration: None,
        }
    }
}

impl<Types: ServerType> ServerBuilder<Types> {
    /// Builds the Server.
    pub fn build(self) -> Server<Types> {
        let configuration = self
            .configuration
            .expect("You did not set the custom configuration.");
        Server {
            account_api: self.account_api.expect("You did not set the call api."),
            block_api: self.block_api.expect("You did not set the call api."),
            call_api: self.call_api.expect("You did not set the call api."),
            construction_api: self
                .construction_api
                .expect("You did not set the construction api."),
            events_api: self.events_api.expect("You did not set the call api."),
            mempool_api: self.mempool_api.expect("You did not set the call api."),
            network_api: self.network_api.expect("You did not set the call api."),
            optional_api: self
                .optional_api
                .expect("You did not set the additional api."),
            search_api: self.search_api.expect("You did not set the call api."),
            node_caller: self.node_caller.expect("You did not set the rpc caller"),
            asserters: Types::init_asserters(&configuration),
            configuration,
        }
    }

    /// Sets the Account API on the builder.
    pub fn account_api(mut self, a: Types::AccountApi) -> Self {
        self.account_api = Some(a);
        self
    }

    /// Sets the Block API on the builder.
    pub fn block_api(mut self, a: Types::BlockApi) -> Self {
        self.block_api = Some(a);
        self
    }

    /// Sets the Call API on the builder.
    pub fn call_api(mut self, a: Types::CallApi) -> Self {
        self.call_api = Some(a);
        self
    }

    /// Sets the construction API on the builder.
    pub fn construction_api(mut self, a: Types::ConstructionApi) -> Self {
        self.construction_api = Some(a);
        self
    }

    /// Sets the Events API on the builder.
    pub fn events_api(mut self, a: Types::EventsApi) -> Self {
        self.events_api = Some(a);
        self
    }

    /// Sets the Mempool API on the builder.
    pub fn mempool_api(mut self, a: Types::MempoolsApi) -> Self {
        self.mempool_api = Some(a);
        self
    }

    /// Sets the Network API on the builder.
    pub fn network_api(mut self, a: Types::NetworkApi) -> Self {
        self.network_api = Some(a);
        self
    }

    /// Sets the optional API on the builder.
    pub fn optional_api(mut self, a: Types::OptionalApi) -> Self {
        self.optional_api = Some(a);
        self
    }

    /// Sets the Search API on the builder.
    pub fn search_api(mut self, a: Types::SearchApi) -> Self {
        self.search_api = Some(a);
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
}

/// The server struct for running the Rosetta server.
pub struct Server<Types: ServerType> {
    /// The Account API endpoints.
    pub account_api: Types::AccountApi,
    /// The Block API endpoints.
    pub block_api: Types::BlockApi,
    /// The Call API endpoints.
    pub call_api: Types::CallApi,
    /// The Construction API endpoints.
    pub construction_api: Types::ConstructionApi,
    /// The Events API endpoints.
    pub events_api: Types::EventsApi,
    /// The Mempool API endpoints.
    pub mempool_api: Types::MempoolsApi,
    /// The network API endpoints.
    pub network_api: Types::NetworkApi,
    /// The search API endpoints.
    pub search_api: Types::SearchApi,
    /// The Optional API endpoints.
    pub optional_api: Types::OptionalApi,
    /// The caller used to interact with the node
    pub node_caller: Types::NodeCaller,
    /// The optional configuration details.
    pub configuration: Configuration<Types::CustomConfig>,
    /// the asserter to be used when asserting requests
    pub asserters: AsserterTable,
}

impl<Types: ServerType> Default for Server<Types> {
    fn default() -> Self {
        let configuration = Types::CustomConfig::load_config();
        Self {
            account_api: Default::default(),
            block_api: Default::default(),
            call_api: Default::default(),
            construction_api: Default::default(),
            events_api: Default::default(),
            mempool_api: Default::default(),
            network_api: Default::default(),
            search_api: Default::default(),
            optional_api: Default::default(),
            node_caller: Types::NodeCaller::from(configuration.clone()),
            asserters: Types::init_asserters(&configuration),
            configuration,
        }
    }
}

impl<Types: ServerType> Server<Types> {
    /// WARNING: Do not use this method outside of Mentat! Use the `mentat` or
    /// `main` macros instead
    #[doc(hidden)]
    pub async fn serve(self, mut app: Router) {
        color_backtrace::install();
        tracing::dispatcher::set_global_default(Types::setup_logging())
            .unwrap_or_else(|err| panic!("Failed to set logger dispatcher: `{err}`"));

        let node_pid = Types::CustomConfig::start_node(&self.configuration);
        let server_pid = Pid::from_u32(std::process::id());

        let addr = SocketAddr::from((self.configuration.address, self.configuration.port));

        Types::middleware(&self.configuration, &mut app);
        app = app
            .route_layer(axum::middleware::from_fn(content_type_middleware))
            .layer(
                tower::ServiceBuilder::new()
                    .layer(Extension(self.configuration))
                    .layer(Extension(node_pid))
                    .layer(Extension(server_pid)),
            )
            .fallback(MentatError::not_found.into_service());

        // TODO this currently writes mentat-server
        // This will be fixed when non basic generic const types stabilize.
        // Or const trait fns stabilize.
        let span = tracing::span!(tracing::Level::DEBUG, env!("CARGO_PKG_NAME"));
        let _enter = span.enter();
        tracing::info!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap_or_else(|err| panic!("Failed to listen on addr `{addr}`: `{err}`."));
        Types::teardown_logging();
    }
}
