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

    fn init_call_api() -> Self::CallApi {
        Self::CallApi::default()
    }

    fn init_construction_api() -> Self::ConstructionApi {
        Self::ConstructionApi::default()
    }

    fn init_data_api() -> Self::DataApi {
        Self::DataApi::default()
    }

    fn init_indexer_api() -> Self::IndexerApi {
        Self::IndexerApi::default()
    }

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

#[derive(Clone)]
pub struct Server<Types: ServerTypes> {
    pub call_api: Types::CallApi,
    pub configuration: Configuration<Types::CustomConfig>,
    pub construction_api: Types::ConstructionApi,
    pub data_api: Types::DataApi,
    pub indexer_api: Types::IndexerApi,
}

impl<Types: ServerTypes> Server<Types> {
    pub fn init() -> Self {
        Server {
            call_api: Types::init_call_api(),
            configuration: Types::load_config(),
            construction_api: Types::init_construction_api(),
            data_api: Types::init_data_api(),
            indexer_api: Types::init_indexer_api(),
        }
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
