pub mod serve_exports {
    pub use std::{borrow::Borrow, net::SocketAddr, sync::Arc};

    pub use axum::{
        extract::{self, ConnectInfo, Extension},
        routing,
        Json,
        Router,
    };
    pub use reqwest::Client;
    pub use tracing;

    pub use crate::{api::*, cache::Cache, conf::*, requests::*, responses::*, server::RpcCaller};
}

#[macro_export]
macro_rules! serve {
    ($server:expr, $conf:ty, $( $cache_inner:ident )?) => {{
        use $crate::server::serve_exports::*;
        let app = serve!(@build $conf, $($cache_inner)?);
        $server.serve(app).await
    }};

    (@routes axum: $app:expr, config: $conf:ty, $(api_group { api: $api:ident, $( route_group { route_base: $route_base:expr, $(route { path: $path:expr, method: $method:ident, req_data: $req:ty, resp_data: $resp:ty, } )* } ) * } ) * )  => {
        $(
            $(
            $(
                #[tracing::instrument(skip(server))]
                async fn $method(
                    Extension(server): Extension<Server<$conf>>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<$req>,
                    Extension(rpc_caller): Extension<RpcCaller>,
                ) -> MentatResponse<$resp> {
                    let c = Caller { ip };
                    let resp = server.$api.$method(c, req_data, &server.configuration.mode, rpc_caller).await;
                    #[cfg(debug_assertions)]
                    tracing::debug!("response {}{} {resp:?}", $route_base, $path);
                    resp
                }
                $app = $app.route(&format!("{}{}", $route_base, $path), routing::post($method));
            )*
            )*
        )*
    };

    (@routes axum: $app:expr, config: $conf:ty, $(api_group { api: $api:ident, $( route_group { route_base: $route_base:expr, $(route { path: $path:expr, method: $method:ident, req_data: $req:ty, resp_data: $resp:ty, cache: $cache:expr } )* } ) * } ) * )  => {
        $(
            $(
            $(
                #[tracing::instrument(skip(server))]
                async fn $method(
                    Extension(server): Extension<Server<$conf>>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<$req>,
                    Extension(rpc_caller): Extension<RpcCaller>,
                ) -> MentatResponse<$resp> {
                    let c = Caller { ip };
                    $cache.get_cached(move || {
                        Box::pin(async move {
                            let resp = server.$api.$method(c, req_data, &server.configuration.mode, rpc_caller).await;
                            #[cfg(debug_assertions)]
                            tracing::debug!("response {}{} {resp:?}", $route_base, $path);
                            resp
                        })

                    })
                    .await
                }
                $app = $app.route(&format!("{}{}", $route_base, $path), routing::post($method));
            )*
            )*
        )*
    };

    (@build $conf:ty, $( $cache_inner:ident )?) => {{
        let mut app = Router::new();

        serve! {@routes
            axum: app,
            config: $conf,

            api_group {
                api: call_api,

                route_group {
                    route_base: "/",

                    route {
                        path: "/call",
                        method: call_call,
                        req_data: CallRequest,
                        resp_data: CallResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
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
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/derive",
                        method: call_derive,
                        req_data: ConstructionDeriveRequest,
                        resp_data: ConstructionDeriveResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/hash",
                        method: call_hash,
                        req_data: ConstructionHashRequest,
                        resp_data: TransactionIdentifierResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/metadata",
                        method: call_metadata,
                        req_data: ConstructionMetadataRequest,
                        resp_data: ConstructionMetadataResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/parse",
                        method: call_parse,
                        req_data: ConstructionParseRequest,
                        resp_data: ConstructionParseResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/payloads",
                        method: call_payloads,
                        req_data: ConstructionPayloadsRequest,
                        resp_data: ConstructionPayloadsResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/preprocess",
                        method: call_preprocess,
                        req_data: ConstructionPreprocessRequest,
                        resp_data: ConstructionPreprocessResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/submit",
                        method: call_submit,
                        req_data: ConstructionSubmitRequest,
                        resp_data: TransactionIdentifierResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
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
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/options",
                        method: call_network_options,
                        req_data: NetworkRequest,
                        resp_data: NetworkOptionsResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/status",
                        method: call_network_status,
                        req_data: NetworkRequest,
                        resp_data: NetworkStatusResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }
                }

                route_group {
                    route_base: "/account",

                    route {
                        path: "/balance",
                        method: call_account_balance,
                        req_data: AccountBalanceRequest,
                        resp_data: AccountBalanceResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/coins",
                        method: call_account_coins,
                        req_data: AccountCoinsRequest,
                        resp_data: AccountCoinsResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }
                }

                route_group {
                    route_base: "/block",

                    route {
                        path: "/",
                        method: call_block,
                        req_data: BlockRequest,
                        resp_data: BlockResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/transaction",
                        method: call_block_transaction,
                        req_data: BlockTransactionRequest,
                        resp_data: BlockTransactionResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }
                }

                route_group {
                    route_base: "/mempool",

                    route {
                        path: "/",
                        method: call_mempool,
                        req_data: NetworkRequest,
                        resp_data: MempoolResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }

                    route {
                        path: "/transaction",
                        method: call_mempool_transaction,
                        req_data: MempoolTransactionRequest,
                        resp_data: MempoolTransactionResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
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
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }
                }

                route_group {
                    route_base: "/search",
                    route {
                    path: "/transactions",
                    method: call_search_transactions,
                    req_data: SearchTransactionsRequest,
                    resp_data: SearchTransactionsResponse,
                        $(cache: Cache::<$cache_inner<_>>::new(Default::default(), None))?
                    }
                }

            }
        }

        app
    }};
}
