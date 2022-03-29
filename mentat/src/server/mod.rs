mod serve;
pub use serve::*;
mod dummy_call;
mod dummy_construction;
mod dummy_data;
mod dummy_indexer;
pub mod logging;
mod middleware_checks;
mod node;

use std::{
    env, fmt,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{extract::Extension, middleware, Router};
pub use node::*;
use tracing::info;

use self::{
    dummy_call::DummyCallApi, dummy_construction::DummyConstructionApi, dummy_data::DummyDataApi,
    dummy_indexer::DummyIndexerApi, middleware_checks::middleware_check,
};
use crate::api::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "MAINNET" => Ok(Self::Mainnet),
            "TESTNET" => Ok(Self::Testnet),
            s => Err(format!("Invalid network id {s}")),
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "MAINNET"),
            Self::Testnet => write!(f, "TESTNET"),
        }
    }
}

#[derive(Clone)]
pub struct Server {
    pub construction_api: Arc<dyn CallerConstructionApi>,
    pub data_api: Arc<dyn CallerDataApi>,
    pub indexer_api: Arc<dyn CallerIndexerApi>,
    pub call_api: Arc<dyn CallerCallApi>,
    pub network: Network,
    pub blockchain: String,
}

impl Default for Server {
    fn default() -> Self {
        let network = match env::var("NETWORK").as_deref() {
            Ok("TESTNET") => Network::Testnet,
            _ => Network::Mainnet,
        };

        Self {
            construction_api: Arc::new(DummyConstructionApi),
            data_api: Arc::new(DummyDataApi),
            indexer_api: Arc::new(DummyIndexerApi),
            call_api: Arc::new(DummyCallApi),
            network,
            blockchain: String::from("UNKNOWN"),
        }
    }
}

impl Server {
    pub fn new(blockchain: String) -> Self {
        let mut new = Self::default();
        new.blockchain = blockchain;
        new
    }

    pub fn with_data_api<T: CallerDataApi + 'static>(
        &mut self,
        mainnet_data_api: T,
        testnet_data_api: T,
    ) {
        match self.network {
            Network::Mainnet => self.with_dyn_data_api(Arc::new(mainnet_data_api)),
            Network::Testnet => self.with_dyn_data_api(Arc::new(testnet_data_api)),
        }
    }

    pub fn with_dyn_data_api(&mut self, data_api: Arc<dyn CallerDataApi>) {
        self.data_api = data_api;
    }

    pub fn with_construction_api<T: CallerConstructionApi + 'static>(
        &mut self,
        mainnet_construction_api: T,
        testnet_construction_api: T,
    ) {
        match self.network {
            Network::Mainnet => self.with_dyn_construction_api(Arc::new(mainnet_construction_api)),
            Network::Testnet => self.with_dyn_construction_api(Arc::new(testnet_construction_api)),
        }
    }

    pub fn with_dyn_construction_api(&mut self, construction_api: Arc<dyn CallerConstructionApi>) {
        self.construction_api = construction_api;
    }

    pub fn with_indexer_api<T: CallerIndexerApi + 'static>(
        &mut self,
        mainnet_indexer_api: T,
        testnet_indexer_api: T,
    ) {
        match self.network {
            Network::Mainnet => self.with_dyn_indexer_api(Arc::new(mainnet_indexer_api)),
            Network::Testnet => self.with_dyn_indexer_api(Arc::new(testnet_indexer_api)),
        }
    }

    pub fn with_dyn_indexer_api(&mut self, indexer_api: Arc<dyn CallerIndexerApi>) {
        self.indexer_api = indexer_api;
    }

    pub fn with_call_api<T: CallerCallApi + 'static>(&mut self, call_api: T) {
        self.with_dyn_call_api(Arc::new(call_api));
    }

    pub fn with_dyn_call_api(&mut self, call_api: Arc<dyn CallerCallApi>) {
        self.call_api = call_api;
    }

    pub async fn serve(
        self,
        mut app: Router,
        address: Ipv4Addr,
        port: u16,
        node: Box<dyn NodeRunner>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        color_backtrace::install();
        logging::setup()?;

        let mode = Mode::default();
        if !mode.is_offline() {
            node.start_node(address.to_string()).await?;
        }

        let client = reqwest::Client::new();

        app = app
            .route_layer(middleware::from_fn(middleware_check))
            .layer(
                tower::ServiceBuilder::new()
                    .layer(Extension(self))
                    .layer(Extension(mode))
                    .layer(Extension(client)),
            );

        let addr = SocketAddr::from((address, port));
        info!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr, _>())
            .await?;

        logging::teardown();
        Ok(())
    }
}
