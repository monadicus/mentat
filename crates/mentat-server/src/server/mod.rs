//! Defines the `Server` methods and launcher for Mentat.

mod builder;
pub use builder::*;
mod middleware;
pub use middleware::*;
mod state;
pub use state::AppState;
mod types;
use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use mentat_types::MentatError;
use sysinfo::{Pid, PidExt};
pub use types::ServerType;

use crate::{api::*, conf::*};

/// The server struct for running the Rosetta server.
pub struct Server<Types: ServerType> {
    /// The Account API endpoints.
    pub account_api: AccountApiRouter<Types::AccountApi>,
    /// The Block API endpoints.
    pub block_api: BlockApiRouter<Types::BlockApi>,
    /// The Call API endpoints.
    pub call_api: CallApiRouter<Types::CallApi>,
    /// The Construction API endpoints.
    pub construction_api: ConstructionApiRouter<Types::ConstructionApi>,
    /// The Events API endpoints.
    pub events_api: EventsApiRouter<Types::EventsApi>,
    /// The Mempool API endpoints.
    pub mempool_api: MempoolApiRouter<Types::MempoolsApi>,
    /// The network API endpoints.
    pub network_api: NetworkApiRouter<Types::NetworkApi>,
    /// The search API endpoints.
    pub search_api: SearchApiRouter<Types::SearchApi>,
    /// The Optional API endpoints.
    pub optional_api: OptionalApiRouter<Types::OptionalApi>,
    /// The optional configuration details.
    pub configuration: Configuration<Types::CustomConfig>,
}

impl<Types: ServerType> Default for Server<Types> {
    fn default() -> Self {
        let configuration = Types::CustomConfig::load_config();
        let node_caller = Arc::new(Types::NodeCaller::from(configuration.clone()));
        Self {
            account_api: AccountApiRouter::<Types::AccountApi>::default_from_caller(
                node_caller.clone(),
            ),
            block_api: BlockApiRouter::<Types::BlockApi>::default_from_caller(node_caller.clone()),
            call_api: CallApiRouter::<Types::CallApi>::default_from_caller(node_caller.clone()),
            construction_api: ConstructionApiRouter::<Types::ConstructionApi>::default_from_caller(
                node_caller.clone(),
            ),
            events_api: EventsApiRouter::<Types::EventsApi>::default_from_caller(
                node_caller.clone(),
            ),
            mempool_api: MempoolApiRouter::<Types::MempoolsApi>::default_from_caller(
                node_caller.clone(),
            ),
            network_api: NetworkApiRouter::<Types::NetworkApi>::default_from_caller(
                node_caller.clone(),
            ),
            search_api: SearchApiRouter::<Types::SearchApi>::default_from_caller(
                node_caller.clone(),
            ),
            optional_api: OptionalApiRouter::<Types::OptionalApi>::default_from_caller(node_caller),
            configuration,
        }
    }
}

impl<Types: ServerType> Server<Types> {
    /// WARNING: Do not use this method outside of Mentat! Use the `mentat` or
    /// `main` macros instead
    #[doc(hidden)]
    pub async fn serve(self) {
        color_backtrace::install();
        Types::setup_logging();

        let node_pid = Types::CustomConfig::start_node(&self.configuration);
        let server_pid = ServerPid(Pid::from_u32(std::process::id()));
        let addr = SocketAddr::from((self.configuration.address, self.configuration.port));

        let state: Arc<AppState<<Types as ServerType>::CustomConfig>> = Arc::new(AppState {
            config: self.configuration.clone(),
            node_pid,
            server_pid,
        });

        let mut app = Router::new();
        app = Types::middleware(&self.configuration, app)
            .nest("/account", self.account_api.to_router())
            .nest("/block", self.block_api.to_router())
            .nest("/call", self.call_api.to_router())
            .nest("/construction", self.construction_api.to_router())
            .nest("/events", self.events_api.to_router())
            .nest("/mempool", self.mempool_api.to_router())
            .nest("/network", self.network_api.to_router())
            .nest("/optional", self.optional_api.to_router())
            .nest("/search", self.search_api.to_router())
            .layer(
                tower::ServiceBuilder::new()
                    .layer(axum::middleware::from_fn(content_type_middleware)),
            )
            .fallback(MentatError::not_found);
        let app = app.with_state(state);

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
