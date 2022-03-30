mod serve;
pub use serve::*;
mod dummy_call;
mod dummy_construction;
mod dummy_data;
mod dummy_indexer;
pub mod logging;
mod middleware_checks;
mod node;
mod rpc_caller;

use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::{extract::Extension, middleware, Router};
pub use node::*;
pub use rpc_caller::RpcCaller;
use tracing::info;

use self::{
    dummy_call::DummyCallApi,
    dummy_construction::DummyConstructionApi,
    dummy_data::DummyDataApi,
    dummy_indexer::DummyIndexerApi,
    middleware_checks::middleware_checks,
};
use crate::{api::*, conf::*};

#[derive(Clone)]
pub struct Server {
    pub call_api: Arc<dyn CallerCallApi>,
    pub configuration: Configuration,
    pub construction_api: Arc<dyn CallerConstructionApi>,
    pub data_api: Arc<dyn CallerDataApi>,
    pub indexer_api: Arc<dyn CallerIndexerApi>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            call_api: Arc::new(DummyCallApi),
            configuration: Default::default(),
            construction_api: Arc::new(DummyConstructionApi),
            data_api: Arc::new(DummyDataApi),
            indexer_api: Arc::new(DummyIndexerApi),
        }
    }
}

impl Server {
    pub fn new<Call, Construction, Data, Indexer>(
        call: Call,
        construct: Construction,
        config: &Path,
        data: Data,
        indexer: Indexer,
    ) -> Self
    where
        Call: CallerCallApi + 'static,
        Construction: CallerConstructionApi + 'static,
        Data: CallerDataApi + 'static,
        Indexer: CallerIndexerApi + 'static,
    {
        Self {
            call_api: Arc::new(call),
            configuration: Configuration::load(config),
            construction_api: Arc::new(construct),
            indexer_api: Arc::new(indexer),
            data_api: Arc::new(data),
        }
    }

    /// WARNING: Do not use this method outside of Mentat! Use the `serve` macro
    /// instead
    #[doc(hidden)]
    pub async fn serve<T: NodeRunner>(
        self,
        mut app: Router,
        node: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        color_backtrace::install();
        logging::setup()?;

        if !self.configuration.mode.is_offline() {
            node.start_node(
                self.configuration.address.to_string(),
                &self.configuration.node_path,
            )
            .await?;
        }

        let rpc_caller = RpcCaller::new(&self.configuration);
        let addr = SocketAddr::from((self.configuration.address, self.configuration.port));

        app = app
            .route_layer(middleware::from_fn(middleware_checks))
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
