//! logic used to generate routes from the user specified types

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

/// mentat routes
const ROUTES: &[RouteGroup] = &[
    RouteGroup {
        field: "optional_api",
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
    },
    RouteGroup {
        field: "call_api",
        route_base: "/call",
        routes: &[Route {
            path: "",
            method: "call_call",
            req_data: Some("UncheckedCallRequest"),
            req_method: "post",
            never_cache: false,
        }],
    },
    RouteGroup {
        field: "construction_api",
        route_base: "/construction",
        routes: &[
            Route {
                path: "/combine",
                method: "call_combine",
                req_data: Some("UncheckedConstructionCombineRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/derive",
                method: "call_derive",
                req_data: Some("UncheckedConstructionDeriveRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/hash",
                method: "call_hash",
                req_data: Some("UncheckedConstructionHashRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/metadata",
                method: "call_metadata",
                req_data: Some("UncheckedConstructionMetadataRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/parse",
                method: "call_parse",
                req_data: Some("UncheckedConstructionParseRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/payloads",
                method: "call_payloads",
                req_data: Some("UncheckedConstructionPayloadsRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/preprocess",
                method: "call_preprocess",
                req_data: Some("UncheckedConstructionPreprocessRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/submit",
                method: "call_submit",
                req_data: Some("UncheckedConstructionSubmitRequest"),
                req_method: "post",
                never_cache: false,
            },
        ],
    },
    RouteGroup {
        field: "network_api",
        route_base: "/network",
        routes: &[
            Route {
                path: "/list",
                method: "call_network_list",
                req_data: Some("UncheckedMetadataRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/options",
                method: "call_network_options",
                req_data: Some("UncheckedNetworkRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/status",
                method: "call_network_status",
                req_data: Some("UncheckedNetworkRequest"),
                req_method: "post",
                never_cache: false,
            },
        ],
    },
    RouteGroup {
        field: "account_api",
        route_base: "/account",
        routes: &[
            Route {
                path: "/balance",
                method: "call_account_balance",
                req_data: Some("UncheckedAccountBalanceRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/coins",
                method: "call_account_coins",
                req_data: Some("UncheckedAccountCoinsRequest"),
                req_method: "post",
                never_cache: false,
            },
        ],
    },
    RouteGroup {
        field: "block_api",
        route_base: "/block",
        routes: &[
            Route {
                path: "",
                method: "call_block",
                req_data: Some("UncheckedBlockRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/transaction",
                method: "call_block_transaction",
                req_data: Some("UncheckedBlockTransactionRequest"),
                req_method: "post",
                never_cache: false,
            },
        ],
    },
    RouteGroup {
        field: "mempool_api",
        route_base: "/mempool",
        routes: &[
            Route {
                path: "",
                method: "call_mempool",
                req_data: Some("UncheckedNetworkRequest"),
                req_method: "post",
                never_cache: false,
            },
            Route {
                path: "/transaction",
                method: "call_mempool_transaction",
                req_data: Some("UncheckedMempoolTransactionRequest"),
                req_method: "post",
                never_cache: false,
            },
        ],
    },
    RouteGroup {
        field: "events_api",
        route_base: "/events",
        routes: &[Route {
            path: "/blocks",
            method: "call_events_blocks",
            req_data: Some("UncheckedEventsBlocksRequest"),
            req_method: "post",
            never_cache: false,
        }],
    },
    RouteGroup {
        field: "search_api",
        route_base: "/search",
        routes: &[Route {
            path: "/transactions",
            method: "call_search_transactions",
            req_data: Some("UncheckedSearchTransactionsRequest"),
            req_method: "post",
            never_cache: false,
        }],
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
    field: &'static str,
    route_base: &'static str,
    routes: &'static [Route],
}

/// builds the routes for rosetta and any optional mentat routes the user
/// specified
pub fn build_routes(server_type: &Ident, cache_type: Option<&Ident>) -> TokenStream2 {
    let mut out = TokenStream2::new();

    for route_group in ROUTES {
        let api_field = Ident::new(route_group.field, Span::call_site());
        for route in route_group.routes {
            let method = Ident::new(route.method, Span::call_site());
            let req_data = route.req_data.map(|d| Ident::new(d, Span::call_site()));
            let req_method = Ident::new(route.req_method, Span::call_site());
            let r = match cache_type {
                Some(cacher) if !route.never_cache => build_cached_route(
                    server_type,
                    &api_field,
                    route_group.route_base,
                    route.path,
                    &method,
                    req_data,
                    &req_method,
                    cacher,
                ),
                _ => build_route(
                    server_type,
                    &api_field,
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

    out
}

/// builds a mentat route without caching enabled
fn build_route(
    server_type: &Ident,
    api_field: &Ident,
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
            Json(req_data): Json<Option<#r>>,
            Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            Extension(rpc_caller): Extension<RpcCaller>,
        );
        req_trace = quote!(
            tracing::info!("{:#?}", req_data);
            tracing::info!("{:#?}", conf);
        );
        method_args = quote!(&asserter, req_data, &conf.mode, rpc_caller);
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
        method_args = quote!(&conf.mode, rpc_caller, server_pid, node_pid);
    }
    quote!(
        let api = server.#api_field.clone();
        let asserter = server.asserters.#api_field.clone();
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
    api_field: &Ident,
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
            Json(req_data): Json<Option<#r>>,
            Extension(conf): Extension<Configuration<<#server_type as ServerType>::CustomConfig>>,
            Extension(rpc_caller): Extension<RpcCaller>,
        );
        req_trace = quote!(
            tracing::info!("{:#?}", req_data);
            tracing::info!("{:#?}", conf);
        );
        method_args = quote!(&asserter, req_data, &conf.mode, rpc_caller);
    } else {
        req_input = quote!(
            Extension(server_pid): Extension<Pid>,
            Extension(node_pid): Extension<NodePid>,
        );
        req_trace = quote! (
            tracing::info!("{}", server_pid);
            tracing::info!("{}", node_pid.0);
        );
        method_args = quote!(server_pid, node_pid);
    }
    quote!(
        let api = server.#api_field.clone();
        let asserter = server.asserters.#api_field.clone();
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
