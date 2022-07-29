//! logic used to generate routes from the user specified types

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

/// mentat routes
const ROUTES: &[ApiGroup] = &[
    ApiGroup {
        api: "optional_api",
        route_groups: &[RouteGroup {
            route_base: "/optional",
            routes: &[
                Route {
                    path: "/health",
                    method: "call_health",
                    req_data: None,
                    req_method: "get",
                    never_cache: true,
                },
                Route {
                    path: "/synced",
                    method: "call_synced",
                    req_data: None,
                    req_method: "get",
                    never_cache: true,
                },
            ],
        }],
    },
    ApiGroup {
        api: "call_api",
        route_groups: &[RouteGroup {
            route_base: "/",
            routes: &[Route {
                path: "call",
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
                        path: "",
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
                        path: "",
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

/// a mentat route
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug)]
struct Route {
    path: &'static str,
    method: &'static str,
    req_data: Option<&'static str>,
    req_method: &'static str,
    never_cache: bool,
}

/// a group of endpoints for rosetta
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug)]
struct RouteGroup {
    route_base: &'static str,
    routes: &'static [Route],
}

/// a base rosetta endpoint and its corresponding route groups
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug)]
struct ApiGroup {
    api: &'static str,
    route_groups: &'static [RouteGroup],
}

/// builds the routes for rosetta and any optional mentat routes the user
/// specified
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
                    Some(cacher) if !route.never_cache => build_cached_route(
                        server_type,
                        &api,
                        route_group.route_base,
                        route.path,
                        &method,
                        req_data,
                        &req_method,
                        cacher,
                    ),
                    _ => build_route(
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

/// builds a mentat route without caching enabled
fn build_route(
    server_type: &Ident,
    api: &Ident,
    route_base: &str,
    path: &str,
    method: &Ident,
    req_data: Option<Ident>,
    req_method: &Ident,
) -> TokenStream2 {
    let req_input;
    let req_trace;
    let method_args;
    if let Some(r) = &req_data {
        req_input = quote!(
            Json(req_data): Json<#r>,
            Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            Extension(rpc_caller): Extension<RpcCaller>,
        );
        req_trace = quote!(
            tracing::info!("{:#?}", req_data);
            tracing::info!("{:#?}", conf);
        );
        method_args = quote!(&conf.asserter, req_data, &conf.mode, rpc_caller);
    } else {
        req_input = quote!(
            Extension(server_pid): Extension<Pid>,
            Extension(node_pid): Extension<NodePid>,
            Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            Extension(rpc_caller): Extension<RpcCaller>
        );
        req_trace = quote! (
            tracing::info!("{}", server_pid);
            tracing::info!("{}", node_pid.0);
        );
        method_args = quote!(&conf.mode, rpc_caller, server_pid, node_pid,);
    }
    quote!(
        let api = server.#api.clone();
        let #method = move |
            ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
            #req_input
            | {
                ::std::boxed::Box::pin(async move {
                    let c = Caller { ip };
                    tracing::info!("{:#?}", c);
                    #req_trace
                    let resp = api.#method(c, #method_args).await;
                    #[cfg(debug_assertions)]
                    tracing::debug!("response {}{} {:?}", #route_base, #path, resp);
                    resp
                })
            }.instrument(tracing::info_span!(stringify!(#method)));

        app = app.route(concat!(#route_base, #path), routing::#req_method(#method));
    )
}

#[allow(clippy::too_many_arguments)]
/// builds a mentat route with caching enabled
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
    let req_input;
    let req_trace;
    let method_args;
    if let Some(r) = &req_data {
        req_input = quote!(
            Json(req_data): Json<#r>,
            Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            Extension(rpc_caller): Extension<RpcCaller>,
        );
        req_trace = quote!(
            tracing::info!("{:#?}", req_data);
            tracing::info!("{:#?}", conf);
        );
        method_args = quote!(&conf.asserter, req_data, &conf.mode, rpc_caller);
    } else {
        req_input = quote!(
            Extension(server_pid): Extension<Pid>,
            Extension(node_pid): Extension<NodePid>,
        );
        req_trace = quote! (
            tracing::info!("{}", server_pid);
            tracing::info!("{}", node_pid.0);
        );
        method_args = quote!(server_pid, node_pid,);
    }
    quote!(
        let api = server.#api.clone();
        let cache = Cache::<#cacher<_>>::new(::std::default::Default::default(), ::std::option::Option::None);

        let #method = move |
                ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                #req_input
            | {
                Box::pin(async move {
                    let c = Caller { ip };
                    tracing::info!("{:#?}", c);
                    #req_trace
                    cache.get_cached(move || {
                        std::boxed::Box::pin(async move {
                            let resp = api.#method(c, #method_args).await;
                            #[cfg(debug_assertions)]
                            tracing::debug!("response {}{} {:?}", #route_base, #path, resp);
                            resp
                        })

                    })
                    .await
                })
            }.instrument(tracing::info_span!(stringify!(#method)));

        app = app.route(concat!(#route_base, #path), routing::#req_method(#method));
    )
}
