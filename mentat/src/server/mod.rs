use serde::de::DeserializeOwned;
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

pub trait ServerType: Sized + 'static {
    type AdditionalApi: AdditionalApi;
    type CallApi: CallerCallApi;
    type CustomConfig: DeserializeOwned + NodeConf;
    type ConstructionApi: CallerConstructionApi;
    type DataApi: CallerDataApi;
    type IndexerApi: CallerIndexerApi;

    fn load_config() -> Configuration<Self::CustomConfig> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("Expected usage: <{}> <configuration file>", args[0]);
            std::process::exit(1);
        }

        let path = std::path::PathBuf::from(&args[1]);
        Configuration::load(&path)
    }
}

pub struct ServerBuilder<Types: ServerType> {
    additional_api: Option<Types::AdditionalApi>,
    call_api: Option<Types::CallApi>,
    configuration: Option<Configuration<Types::CustomConfig>>,
    construction_api: Option<Types::ConstructionApi>,
    data_api: Option<Types::DataApi>,
    indexer_api: Option<Types::IndexerApi>,
}

impl<Types: ServerType> Default for ServerBuilder<Types> {
    fn default() -> Self {
        Self {
            additional_api: None,
            call_api: None,
            configuration: None,
            construction_api: None,
            data_api: None,
            indexer_api: None,
        }
    }
}

impl<Types: ServerType> ServerBuilder<Types> {
    pub fn build(self) -> Server<Types> {
        Server {
            additional_api: self
                .additional_api
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

    pub fn additional_api(mut self, a: Types::AdditionalApi) -> Self {
        self.additional_api = Some(a);
        self
    }

    pub fn call_api(mut self, a: Types::CallApi) -> Self {
        self.call_api = Some(a);
        self
    }

    pub fn custom_configuration_from_arg(self) -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("Expected usage: <{}> <configuration file>", args[0]);
            std::process::exit(1);
        }

        let path = std::path::PathBuf::from(&args[1]);
        self.custom_configuration(&path)
    }

    pub fn custom_configuration(mut self, path: &std::path::Path) -> Self {
        self.configuration = Some(Configuration::load(path));
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

pub struct Server<Types: ServerType> {
    pub additional_api: Types::AdditionalApi,
    pub call_api: Types::CallApi,
    pub configuration: Configuration<Types::CustomConfig>,
    pub construction_api: Types::ConstructionApi,
    pub data_api: Types::DataApi,
    pub indexer_api: Types::IndexerApi,
}

impl<Types: ServerType> Default for Server<Types> {
    fn default() -> Self {
        Self {
            additional_api: Default::default(),
            call_api: Default::default(),
            configuration: <Types>::load_config(),
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
    pub async fn serve(self, mut app: Router) -> Result<(), Box<dyn std::error::Error>> {
        color_backtrace::install();
        logging::setup()?;

        if !self.configuration.mode.is_offline() {
            Types::CustomConfig::start_node(&self.configuration)?;
        }

        let rpc_caller = RpcCaller::new(&self.configuration);
        let addr = SocketAddr::from((self.configuration.address, self.configuration.port));

        app = app
            .route_layer(middleware::from_fn(middleware_checks::<Types>))
            .layer(
                tower::ServiceBuilder::new()
                    .layer(Extension(self.configuration))
                    .layer(Extension(rpc_caller)),
            );

        info!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await?;

        logging::teardown();
        Ok(())
    }
}
