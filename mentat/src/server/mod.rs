mod serve;
use serde::{de::DeserializeOwned, Serialize};
pub use serve::*;
mod dummy_call;
mod dummy_construction;
mod dummy_data;
mod dummy_indexer;
pub mod logging;
mod middleware_checks;
mod rpc_caller;

use std::net::SocketAddr;

use axum::{extract::Extension, middleware, Router};
pub use rpc_caller::RpcCaller;
use tracing::info;

use self::middleware_checks::middleware_checks;
use crate::{api::*, conf::*};

pub trait ServerTypes: Clone + Send + Sync + 'static {
    type CallApi: Clone + CallerCallApi + Send + Sync + 'static;
    type CustomConfig: Clone + DeserializeOwned + NodeConf + Send + Serialize + Sync + 'static;
    type ConstructionApi: Clone + CallerConstructionApi + Send + Sync + 'static;
    type DataApi: Clone + CallerDataApi + Send + Sync + 'static;
    type IndexerApi: Clone + CallerIndexerApi + Send + Sync + 'static;
}

#[derive(Clone)]
pub struct Server<Types: ServerTypes> {
    pub call_api: Types::CallApi,
    pub configuration: Configuration<Types::CustomConfig>,
    pub construction_api: Types::ConstructionApi,
    pub data_api: Types::DataApi,
    pub indexer_api: Types::IndexerApi,
}

impl<Types: ServerTypes> Server<Types> {
    pub fn builder() -> ServerBuilder<Types> {
        Default::default()
    }

    /// WARNING: Do not use this method outside of Mentat! Use the `serve` macro
    /// instead
    #[doc(hidden)]
    pub async fn serve(self, mut app: Router) -> Result<(), Box<dyn std::error::Error>> {
        color_backtrace::install();
        logging::setup()?;

        if !self.configuration.mode.is_offline() {
            self.configuration.start_node().await?;
        }

        let rpc_caller = RpcCaller::new(&self.configuration);
        let addr = SocketAddr::from((self.configuration.address, self.configuration.port));

        app = app
            .route_layer(middleware::from_fn(middleware_checks::<Types>))
            .layer(
                tower::ServiceBuilder::new()
                    .layer(Extension(self))
                    .layer(Extension(rpc_caller)),
            );

        info!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
            .await?;

        logging::teardown();
        Ok(())
    }
}

pub struct ServerBuilder<Types: ServerTypes> {
    call_api: Option<Types::CallApi>,
    configuration: Option<Configuration<Types::CustomConfig>>,
    construction_api: Option<Types::ConstructionApi>,
    data_api: Option<Types::DataApi>,
    indexer_api: Option<Types::IndexerApi>,
}

impl<Types: ServerTypes> Default for ServerBuilder<Types> {
    fn default() -> Self {
        Self {
            call_api: None,
            configuration: None,
            construction_api: None,
            data_api: None,
            indexer_api: None,
        }
    }
}

impl<Types: ServerTypes> ServerBuilder<Types> {
    pub fn build(self) -> Server<Types> {
        Server {
            call_api: self.call_api.expect("You did not set the call api."),
            configuration: self
                .configuration
                .expect("You did not set the custom configuration."),
            construction_api: self
                .construction_api
                .expect("You did not set the construction api."),
            data_api: self.data_api.expect("You did not set the data api."),
            indexer_api: self.indexer_api.expect("You did not set the indxer api."),
        }
    }

    pub fn call_api(mut self, a: Types::CallApi) -> Self {
        self.call_api = Some(a);
        self
    }

    pub fn custom_configuration(mut self, config: Types::CustomConfig) -> Self {
        self.configuration = Some(Configuration::new(config));
        self
    }

    pub fn construction_api(mut self, a: Types::ConstructionApi) -> Self {
        self.construction_api = Some(a);
        self
    }

    pub fn data_api(mut self, a: Types::DataApi) -> Self {
        self.data_api = Some(a);
        self
    }

    pub fn indexer_api(mut self, a: Types::IndexerApi) -> Self {
        self.indexer_api = Some(a);
        self
    }
}
