use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

const ROUTES: &[ApiGroup] = &[
    ApiGroup {
        api: "additional_api",
        route_groups: &[RouteGroup {
            route_base: "/",
            routes: &[Route {
                path: "health",
                method: "health",
                req_data: None,
                req_method: "get",
                never_cache: true,
            }],
        }],
    },
    ApiGroup {
        api: "call_api",
        route_groups: &[RouteGroup {
            route_base: "/",
            routes: &[Route {
                path: "/call",
                method: "call_call",
                req_data: Some("CallRequest"),
                req_method: "post",
                never_cache: false,
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
                    req_data: Some("ConstructionCombineRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/derive",
                    method: "call_derive",
                    req_data: Some("ConstructionDeriveRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/hash",
                    method: "call_hash",
                    req_data: Some("ConstructionHashRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/metadata",
                    method: "call_metadata",
                    req_data: Some("ConstructionMetadataRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/parse",
                    method: "call_parse",
                    req_data: Some("ConstructionParseRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/payloads",
                    method: "call_payloads",
                    req_data: Some("ConstructionPayloadsRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/preprocess",
                    method: "call_preprocess",
                    req_data: Some("ConstructionPreprocessRequest"),
                    req_method: "post",
                    never_cache: false,
                },
                Route {
                    path: "/submit",
                    method: "call_submit",
                    req_data: Some("ConstructionSubmitRequest"),
                    req_method: "post",
                    never_cache: false,
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
                        req_data: Some("MetadataRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                    Route {
                        path: "/options",
                        method: "call_network_options",
                        req_data: Some("NetworkRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                    Route {
                        path: "/status",
                        method: "call_network_status",
                        req_data: Some("NetworkRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                ],
            },
            RouteGroup {
                route_base: "/account",
                routes: &[
                    Route {
                        path: "/balance",
                        method: "call_account_balance",
                        req_data: Some("AccountBalanceRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                    Route {
                        path: "/coins",
                        method: "call_account_coins",
                        req_data: Some("AccountCoinsRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                ],
            },
            RouteGroup {
                route_base: "/block",
                routes: &[
                    Route {
                        path: "/",
                        method: "call_block",
                        req_data: Some("BlockRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                    Route {
                        path: "/transaction",
                        method: "call_block_transaction",
                        req_data: Some("BlockTransactionRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                ],
            },
            RouteGroup {
                route_base: "/mempool",
                routes: &[
                    Route {
                        path: "/",
                        method: "call_mempool",
                        req_data: Some("NetworkRequest"),
                        req_method: "post",
                        never_cache: false,
                    },
                    Route {
                        path: "/transaction",
                        method: "call_mempool_transaction",
                        req_data: Some("MempoolTransactionRequest"),
                        req_method: "post",
                        never_cache: false,
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
                    req_data: Some("EventsBlocksRequest"),
                    req_method: "post",
                    never_cache: false,
                }],
            },
            RouteGroup {
                route_base: "/search",
                routes: &[Route {
                    path: "/transactions",
                    method: "call_search_transactions",
                    req_data: Some("SearchTransactionsRequest"),
                    req_method: "post",
                    never_cache: false,
                }],
            },
        ],
    },
];

struct Route {
    path: &'static str,
    method: &'static str,
    req_data: Option<&'static str>,
    req_method: &'static str,
    never_cache: bool,
}

struct RouteGroup {
    route_base: &'static str,
    routes: &'static [Route],
}

struct ApiGroup {
    api: &'static str,
    route_groups: &'static [RouteGroup],
}

pub fn build_routes(server_type: &Ident, cache_type: Option<&Ident>) -> TokenStream2 {
    let mut out = TokenStream2::new();

    for api_group in ROUTES {
        let api = Ident::new(api_group.api, Span::call_site());
        for route_group in api_group.route_groups {
            for route in route_group.routes {
                let method = Ident::new(route.method, Span::call_site());
                let req_data = route.req_data.map(|d| Ident::new(d, Span::call_site()));
                let req_method = Ident::new(route.req_method, Span::call_site());
                let r = match cache_type {
                    Some(_) if route.never_cache => build_route(
                        server_type,
                        &api,
                        route_group.route_base,
                        route.path,
                        &method,
                        req_data,
                        &req_method,
                    ),
                    Some(cacher) => build_cached_route(
                        server_type,
                        &api,
                        route_group.route_base,
                        route.path,
                        &method,
                        req_data,
                        &req_method,
                        cacher,
                    ),
                    None => build_route(
                        server_type,
                        &api,
                        route_group.route_base,
                        route.path,
                        &method,
                        req_data,
                        &req_method,
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
    req_data: Option<Ident>,
    req_method: &Ident,
) -> TokenStream2 {
    match req_data {
        Some(data) => quote!(
            let api = server.#api.clone();
            let #method = move |
                ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                Json(req_data): Json<#data>,
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

            app = app.route(concat!(#route_base, #path), routing::#req_method(#method));
        ),
        None => quote!(
            let api = server.#api.clone();
            let #method = move |
                    ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                    Extension(rpc_caller): Extension<RpcCaller>
                | {
                    ::std::boxed::Box::pin(async move {
                        let c = Caller { ip };
                        let resp = api.#method(c, rpc_caller).await;
                        #[cfg(debug_assertions)]
                        tracing::debug!("response {}{} {:?}", #route_base, #path, resp);
                        resp
                    })
                }.instrument(tracing::info_span!(stringify!(#method)));

            app = app.route(concat!(#route_base, #path), routing::#req_method(#method));
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_cached_route(
    server_type: &Ident,
    api: &Ident,
    route_base: &str,
    path: &str,
    method: &Ident,
    req_data: Option<Ident>,
    req_method: &Ident,
    cacher: &Ident,
) -> TokenStream2 {
    match req_data {
        Some(data) => quote!(
            let api = server.#api.clone();
            let cache = Cache::<#cacher<_>>::new(::std::default::Default::default(), ::std::option::Option::None);

            let #method = move |
                    ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                    extract::Json(req_data): Json<#data>,
                    extract::Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
                    extract::Extension(rpc_caller): Extension<RpcCaller>
                | {
                    Box::pin(async move {
                        let c = Caller { ip };
                        cache.get_cached(move || {
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

            app = app.route(concat!(#route_base, #path), routing::#req_method(#method));
        ),
        None => quote!(
            let api = server.#api.clone();
            let cache = Cache::<#cacher<_>>::new(::std::default::Default::default(), ::std::option::Option::None);

            let #method = move |
                    ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                    extract::Extension(rpc_caller): Extension<RpcCaller>
                | {
                    Box::pin(async move {
                        let c = Caller { ip };
                        cache.get_cached(move || {
                            std::boxed::Box::pin(async move {
                                let resp = api.#method(c, rpc_caller).await;
                                #[cfg(debug_assertions)]
                                tracing::debug!("response {}{} {:?}", #route_base, #path, resp);
                                resp
                            })

                        })
                        .await
                    })
                }.instrument(tracing::info_span!(stringify!(#method)));

            app = app.route(concat!(#route_base, #path), routing::#req_method(#method));
        ),
    }
}
