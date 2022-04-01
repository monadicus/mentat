use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

const ROUTES: &[ApiGroup] = &[
    ApiGroup {
        api: "call_api",
        route_groups: &[RouteGroup {
            route_base: "/",
            routes: &[Route {
                path: "/call",
                method: "call_call",
                req_data: "CallRequest",
            }],
        }],
    },
    ApiGroup {
        api: "construction_api",
        route_groups: &[RouteGroup {
            route_base: "/construction",
            routes: &[
                Route {
                    path: "/combine",
                    method: "call_combine",
                    req_data: "ConstructionCombineRequest",
                },
                Route {
                    path: "/derive",
                    method: "call_derive",
                    req_data: "ConstructionDeriveRequest",
                },
                Route {
                    path: "/hash",
                    method: "call_hash",
                    req_data: "ConstructionHashRequest",
                },
                Route {
                    path: "/metadata",
                    method: "call_metadata",
                    req_data: "ConstructionMetadataRequest",
                },
                Route {
                    path: "/parse",
                    method: "call_parse",
                    req_data: "ConstructionParseRequest",
                },
                Route {
                    path: "/payloads",
                    method: "call_payloads",
                    req_data: "ConstructionPayloadsRequest",
                },
                Route {
                    path: "/preprocess",
                    method: "call_preprocess",
                    req_data: "ConstructionPreprocessRequest",
                },
                Route {
                    path: "/submit",
                    method: "call_submit",
                    req_data: "ConstructionSubmitRequest",
                },
            ],
        }],
    },
    ApiGroup {
        api: "data_api",
        route_groups: &[
            RouteGroup {
                route_base: "/network",
                routes: &[
                    Route {
                        path: "/list",
                        method: "call_network_list",
                        req_data: "MetadataRequest",
                    },
                    Route {
                        path: "/options",
                        method: "call_network_options",
                        req_data: "NetworkRequest",
                    },
                    Route {
                        path: "/status",
                        method: "call_network_status",
                        req_data: "NetworkRequest",
                    },
                ],
            },
            RouteGroup {
                route_base: "/account",
                routes: &[
                    Route {
                        path: "/balance",
                        method: "call_account_balance",
                        req_data: "AccountBalanceRequest",
                    },
                    Route {
                        path: "/coins",
                        method: "call_account_coins",
                        req_data: "AccountCoinsRequest",
                    },
                ],
            },
            RouteGroup {
                route_base: "/block",
                routes: &[
                    Route {
                        path: "/",
                        method: "call_block",
                        req_data: "BlockRequest",
                    },
                    Route {
                        path: "/transaction",
                        method: "call_block_transaction",
                        req_data: "BlockTransactionRequest",
                    },
                ],
            },
            RouteGroup {
                route_base: "/mempool",
                routes: &[
                    Route {
                        path: "/",
                        method: "call_mempool",
                        req_data: "NetworkRequest",
                    },
                    Route {
                        path: "/transaction",
                        method: "call_mempool_transaction",
                        req_data: "MempoolTransactionRequest",
                    },
                ],
            },
        ],
    },
    ApiGroup {
        api: "indexer_api",
        route_groups: &[
            RouteGroup {
                route_base: "/events",
                routes: &[Route {
                    path: "/blocks",
                    method: "call_events_blocks",
                    req_data: "EventsBlocksRequest",
                }],
            },
            RouteGroup {
                route_base: "/search",
                routes: &[Route {
                    path: "/transactions",
                    method: "call_search_transactions",
                    req_data: "SearchTransactionsRequest",
                }],
            },
        ],
    },
];

struct Route {
    path: &'static str,
    method: &'static str,
    req_data: &'static str,
}

struct RouteGroup {
    route_base: &'static str,
    routes: &'static [Route],
}

struct ApiGroup {
    api: &'static str,
    route_groups: &'static [RouteGroup],
}

pub fn build_routes(server_type: &Ident, cacher: Option<&Ident>) -> TokenStream2 {
    let mut out = TokenStream2::new();

    for api_group in ROUTES {
        let api = Ident::new(api_group.api, Span::call_site());
        for route_group in api_group.route_groups {
            for route in route_group.routes {
                let method = Ident::new(route.method, Span::call_site());
                let req_data = Ident::new(route.req_data, Span::call_site());
                let r = match cacher {
                    Some(cacher) => build_cached_route(
                        server_type,
                        &api,
                        route_group.route_base,
                        route.path,
                        &method,
                        &req_data,
                        cacher,
                    ),
                    None => build_route(
                        server_type,
                        &api,
                        route_group.route_base,
                        route.path,
                        &method,
                        &req_data,
                    ),
                };
                out.extend(r);
            }
        }
    }

    out
}

fn build_route(
    server_type: &Ident,
    api: &Ident,
    route_base: &str,
    path: &str,
    method: &Ident,
    req: &Ident,
) -> TokenStream2 {
    quote!(
        let api = server.#api.clone();
        let #method = move |
            ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
            Json(req_data): Json<#req>,
            Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            Extension(rpc_caller): Extension<RpcCaller>
        | {
            ::std::boxed::Box::pin(async move {
                let c = Caller { ip };
                let resp = api.#method(c, req_data, &conf.mode, rpc_caller).await;
                #[cfg(debug_assertions)]
                tracing::debug!("response {}{} {:?}", #route_base, #path, resp);
                resp
            })
        }.instrument(tracing::info_span!(stringify!(#method)));

        app = app.route(concat!(#route_base, #path), routing::post(#method));
    )
}

fn build_cached_route(
    server_type: &Ident,
    api: &Ident,
    route_base: &str,
    path: &str,
    method: &Ident,
    req: &Ident,
    cacher: &Ident,
) -> TokenStream2 {
    quote!(
        let api = server.#api.clone();
        let #method = move |
            ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
            extract::Json(req_data): Json<#req>,
            extract::Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            extract::Extension(rpc_caller): Extension<RpcCaller>
        | {
            Box::pin(async move {
                let c = Caller { ip };
                Cache::<#cacher<_>>::new(::std::default::Default::default(), ::std::option::Option::None).get_cached(move || {
                    std::boxed::Box::pin(async move {
                        let resp = api.#method(c, req_data, &conf.mode, rpc_caller).await;
                        #[cfg(debug_assertions)]
                        tracing::debug!("response {}{} {:?}", #route_base, #path, resp);
                        resp
                    })

                })
                .await
            })
        }.instrument(tracing::info_span!(stringify!(#method)));

        app = app.route(concat!(#route_base, #path), routing::post(#method));
    )
}
