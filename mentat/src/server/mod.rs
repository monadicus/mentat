mod dummy_call;
mod dummy_construction;
mod dummy_data;
mod dummy_indexer;
mod logging;
mod node;
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::extract::{self, ConnectInfo, Extension};
pub use node::*;
use reqwest::Client;
use tracing::info;

use self::{
    dummy_call::DummyCallApi,
    dummy_construction::DummyConstructionApi,
    dummy_data::DummyDataApi,
    dummy_indexer::DummyIndexerApi,
};
use crate::{api::*, requests::*, responses::*};

#[derive(Clone)]
pub enum Network {
    Mainnet,
    Testnet,
}

macro_rules! api_routes {
    (axum: $app:expr, $(api_group { api: $api:ident, $( route_group { route_base: $route_base:expr, $(route { path: $path:expr, method: $method:ident, req_data: $req:ty, resp_data: $resp:ty, } )* } ) * } ) * )  => {
        $(
            $(
            $(
                #[tracing::instrument(skip(server))]
                async fn $method(
                    Extension(server): Extension<Server>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): axum::Json<$req>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentantResponse<$resp> {
                    let c = Caller { ip };
                    let resp = server.$api.$method(c, req_data, &mode, client).await;
                    #[cfg(debug_assertions)]
                    tracing::debug!("response {}{} {resp:?}", $route_base, $path);
                    resp
                }
                $app = $app.route(&format!("{}{}", $route_base, $path), axum::routing::post($method));
            )*
            )*
        )*
    };
}

#[derive(Clone)]
pub struct Server {
    construction_api: Arc<dyn CallerConstructionApi>,
    data_api: Arc<dyn CallerDataApi>,
    indexer_api: Arc<dyn CallerIndexerApi>,
    call_api: Arc<dyn CallerCallApi>,
    network: Network,
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
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Default::default()
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
        address: Ipv4Addr,
        port: u16,
        node: Box<dyn NodeRunner>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        logging::setup()?;

        let mut app = axum::Router::new();

        api_routes! {
            axum: app,

            api_group {
                api: call_api,

                route_group {
                    route_base: "/",

                    route {
                        path: "/call",
                        method: call_call,
                        req_data: CallRequest,
                        resp_data: CallResponse,
                    }
                }
            }

            api_group {
                api: construction_api,

                route_group {
                    route_base: "/construction",

                    route {
                        path: "/combine",
                        method: call_combine,
                        req_data: ConstructionCombineRequest,
                        resp_data: ConstructionCombineResponse,
                    }

                    route {
                        path: "/derive",
                        method: call_derive,
                        req_data: ConstructionDeriveRequest,
                        resp_data: ConstructionDeriveResponse,
                    }

                    route {
                        path: "/hash",
                        method: call_hash,
                        req_data: ConstructionHashRequest,
                        resp_data: TransactionIdentifierResponse,
                    }

                    route {
                        path: "/metadata",
                        method: call_metadata,
                        req_data: ConstructionMetadataRequest,
                        resp_data: ConstructionMetadataResponse,
                    }

                    route {
                        path: "/parse",
                        method: call_parse,
                        req_data: ConstructionParseRequest,
                        resp_data: ConstructionParseResponse,
                    }

                    route {
                        path: "/payloads",
                        method: call_payloads,
                        req_data: ConstructionPayloadsRequest,
                        resp_data: ConstructionPayloadsResponse,
                    }

                    route {
                        path: "/preprocess",
                        method: call_preprocess,
                        req_data: ConstructionPreprocessRequest,
                        resp_data: ConstructionPreprocessResponse,
                    }

                    route {
                        path: "/submit",
                        method: call_submit,
                        req_data: ConstructionSubmitRequest,
                        resp_data: TransactionIdentifierResponse,
                    }
                }
            }

            api_group {
                api: data_api,

                route_group {
                    route_base: "/network",

                    route {
                        path: "/list",
                        method: call_network_list,
                        req_data: MetadataRequest,
                        resp_data: NetworkListResponse,
                    }

                    route {
                        path: "/options",
                        method: call_network_options,
                        req_data: NetworkRequest,
                        resp_data: NetworkOptionsResponse,
                    }

                    route {
                        path: "/status",
                        method: call_network_status,
                        req_data: NetworkRequest,
                        resp_data: NetworkStatusResponse,
                    }
                }

                route_group {
                    route_base: "/account",

                    route {
                        path: "/balance",
                        method: call_account_balance,
                        req_data: AccountBalanceRequest,
                        resp_data: AccountBalanceResponse,
                    }

                    route {
                        path: "/coins",
                        method: call_account_coins,
                        req_data: AccountCoinsRequest,
                        resp_data: AccountCoinsResponse,
                    }
                }

                route_group {
                    route_base: "/block",

                    route {
                        path: "/",
                        method: call_block,
                        req_data: BlockRequest,
                        resp_data: BlockResponse,
                    }

                    route {
                        path: "/transaction",
                        method: call_block_transaction,
                        req_data: BlockTransactionRequest,
                        resp_data: BlockTransactionResponse,
                    }
                }

                route_group {
                    route_base: "/mempool",

                    route {
                        path: "/",
                        method: call_mempool,
                        req_data: NetworkRequest,
                        resp_data: MempoolResponse,
                    }

                    route {
                        path: "/transaction",
                        method: call_mempool_transaction,
                        req_data: MempoolTransactionRequest,
                        resp_data: MempoolTransactionResponse,
                    }
                }
            }

            api_group {
                api: indexer_api,

                route_group {
                    route_base: "/events",

                        route {
                        path: "/blocks",
                        method: call_events_blocks,
                        req_data: EventsBlocksRequest,
                        resp_data: EventsBlocksResponse,
                    }
                }

                route_group {
                    route_base: "/search",
                     route {
                     path: "/transactions",
                     method: call_search_transactions,
                     req_data: SearchTransactionsRequest,
                     resp_data: SearchTransactionsResponse,
                     }
                }

            }
        }

        let mode = Mode::default();
        if !mode.is_offline() {
            node.start_node(address.to_string()).await?;
        }

        let client = reqwest::Client::new();

        app = app.layer(
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
