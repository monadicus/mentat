mod dummy_call;
mod dummy_construction;
mod dummy_data;
mod dummy_indexer;

mod bitcoin_call;
mod bitcoin_construction;
mod bitcoin_data;
mod bitcoin_indexer;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use rocket::serde::json::Json;
use rocket::{post, routes, Config, State};

use crate::{api::*, errors::*, requests::*, responses::*};

use self::{
    bitcoin_call::BitcoinCallApi, bitcoin_construction::BitcoinConstructionApi,
    bitcoin_data::BitcoinDataApi, bitcoin_indexer::BitcoinIndexerApi, dummy_call::DummyCallApi,
    dummy_construction::DummyConstructionApi, dummy_data::DummyDataApi,
    dummy_indexer::DummyIndexerApi,
};

pub struct Server {
    construction_api: Arc<dyn ConstructionApi>,
    data_api: Arc<dyn DataApi>,
    indexer_api: Arc<dyn IndexerApi>,
    call_api: Arc<dyn CallApi>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            construction_api: Arc::new(DummyConstructionApi),
            data_api: Arc::new(DummyDataApi),
            indexer_api: Arc::new(DummyIndexerApi),
            call_api: Arc::new(DummyCallApi),
        }
    }
}

macro_rules! api_routes {
    (rocket: $rocket:expr, $(api_group { api: $api:ident, $( route_group { route_base: $route_base:expr, $(route { path: $path:expr, method: $method:ident, req_data: $req:ty, resp_data: $resp:ty, } )* } ) * } ) * )  => {
	$(
	    $(
		$(
		    #[post($path, format = "json", data = "<req_data>")]
		    async fn $method(
			server: &State<Server>,
			ip: SocketAddr,
			req_data: Json<$req>
		    ) -> Response<$resp> {
			let c = Caller { ip };
			server.$api.$method(c, req_data.into_inner()).await
		    }

		    $rocket = $rocket.mount($route_base, routes![$method]);
		)*
	    )*
	)*
    }
}

impl Server {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bitcoin() -> Self {
        Server {
            construction_api: Arc::new(BitcoinConstructionApi::default()),
            data_api: Arc::new(BitcoinDataApi::default()),
            indexer_api: Arc::new(BitcoinIndexerApi::default()),
            call_api: Arc::new(BitcoinCallApi::default()),
        }
    }

    pub fn with_data_api<T: DataApi + 'static>(&mut self, data_api: T) {
        self.with_dyn_data_api(Arc::new(data_api));
    }

    pub fn with_dyn_data_api(&mut self, data_api: Arc<dyn DataApi>) {
        self.data_api = data_api;
    }

    pub fn with_construction_api<T: ConstructionApi + 'static>(&mut self, construction_api: T) {
        self.with_dyn_construction_api(Arc::new(construction_api));
    }

    pub fn with_dyn_construction_api(&mut self, construction_api: Arc<dyn ConstructionApi>) {
        self.construction_api = construction_api;
    }

    pub fn with_indexer_api<T: IndexerApi + 'static>(&mut self, indexer_api: T) {
        self.with_dyn_indexer_api(Arc::new(indexer_api));
    }

    pub fn with_dyn_indexer_api(&mut self, indexer_api: Arc<dyn IndexerApi>) {
        self.indexer_api = indexer_api;
    }

    pub fn with_call_api<T: CallApi + 'static>(&mut self, call_api: T) {
        self.with_dyn_call_api(Arc::new(call_api));
    }

    pub fn with_dyn_call_api(&mut self, call_api: Arc<dyn CallApi>) {
        self.call_api = call_api;
    }

    pub async fn serve(self, address: Ipv4Addr, port: u16) {
        let config = Config {
            port,
            address: address.into(),
            ..Config::default()
        };
        let mut rocket = rocket::custom(&config);

        api_routes! {
            rocket: rocket,

            api_group {
                api: data_api,

                route_group {
                    route_base: "/network",

                    route {
                        path: "/list",
                        method: network_list,
                        req_data: MetadataRequest,
                        resp_data: NetworkListResponse,
                    }

                    route {
                        path: "/options",
                        method: network_options,
                        req_data: NetworkRequest,
                        resp_data: NetworkOptionsResponse,
                    }

                    route {
                        path: "/status",
                        method: network_status,
                        req_data: NetworkRequest,
                        resp_data: NetworkStatusResponse,
                    }
                }

                route_group {
                    route_base: "/account",

                    route {
                        path: "/balance",
                        method: account_balance,
                        req_data: AccountBalanceRequest,
                        resp_data: AccountBalanceResponse,
                    }

                    route {
                        path: "/coins",
                        method: account_coins,
                        req_data: AccountCoinsRequest,
                        resp_data: AccountCoinsResponse,
                    }
                }

                route_group {
                    route_base: "/block",

                    route {
                        path: "/",
                        method: block,
                        req_data: BlockRequest,
                        resp_data: BlockResponse,
                    }

                    route {
                        path: "/transaction",
                        method: block_transaction,
                        req_data: BlockTransactionRequest,
                        resp_data: BlockTransactionResponse,
                    }
                }

                route_group {
                    route_base: "/mempool",

                    route {
                        path: "/",
                        method: mempool,
                        req_data: NetworkRequest,
                        resp_data: MempoolResponse,
                    }

                    route {
                        path: "/transaction",
                        method: mempool_transaction,
                        req_data: MempoolTransactionRequest,
                        resp_data: MempoolTransactionResponse,
                    }
                }
            }

            api_group {
                api: construction_api,

                route_group {
                    route_base: "/construction",

                    route {
                        path: "/combine",
                        method: combine,
                        req_data: ConstructionCombineRequest,
                        resp_data: ConstructionCombineResponse,
                    }

                    route {
                        path: "/derive",
                        method: derive,
                        req_data: ConstructionDeriveRequest,
                        resp_data: ConstructionDeriveResponse,
                    }

                    route {
                        path: "/hash",
                        method: hash,
                        req_data: ConstructionHashRequest,
                        resp_data: TransactionIdentifierResponse,
                    }

                    route {
                        path: "/metadata",
                        method: metadata,
                        req_data: ConstructionMetadataRequest,
                        resp_data: ConstructionMetadataResponse,
                    }

                    route {
                        path: "/parse",
                        method: parse,
                        req_data: ConstructionParseRequest,
                        resp_data: ConstructionParseResponse,
                    }

                    route {
                        path: "/payloads",
                        method: payloads,
                        req_data: ConstructionPayloadsRequest,
                        resp_data: ConstructionPayloadsResponse,
                    }

                    route {
                        path: "/preprocess",
                        method: preprocess,
                        req_data: ConstructionPreprocessRequest,
                        resp_data: ConstructionPreprocessResponse,
                    }

                    route {
                        path: "/submit",
                        method: submit,
                        req_data: ConstructionSubmitRequest,
                        resp_data: TransactionIdentifierResponse,
                    }
                }
            }

            api_group {
                api: indexer_api,

                    route_group {
                    route_base: "/events",

                    route {
                        path: "/blocks",
                        method: events_blocks,
                        req_data: EventsBlocksRequest,
                        resp_data: EventsBlocksResponse,
                    }
                }

                route_group {
                    route_base: "/search",
                    route {
                        path: "/transactions",
                        method: search_transactions,
                        req_data: SearchTransactionsRequest,
                        resp_data: SearchTransactionsResponse,
                    }
                }
            }

            api_group {
                api: call_api,

                route_group {
                    route_base: "/call",

                    route {
                        path: "/",
                        method: call,
                        req_data: CallRequest,
                        resp_data: CallResponse,
                    }
                }
            }
        }

        rocket
            .manage(self)
            .ignite()
            .await
            .expect("Failed to iginite rocket")
            .launch()
            .await
            .expect("Failed to start server");
    }
}
