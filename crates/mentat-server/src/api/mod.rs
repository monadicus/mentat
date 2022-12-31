//! Defines the different API traits needed for Rosetta.

mod account;
pub use account::*;
mod block;
pub use block::*;
mod call;
pub use call::*;
mod construction;
pub use construction::*;
mod events;
pub use events::*;
mod mempool;
pub use mempool::*;
mod network;
pub use network::*;
mod optional;
pub use optional::*;
mod search;

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use mentat_asserter::Asserter;
use mentat_types::*;
pub use search::*;

use crate::{
    conf::{Configuration, Mode, NodeConf},
    server::AppState,
};

/// ApiRouter defines the required methods for binding the api requests
/// to a responses for the specified api trait.
/// The ApiRouter implementation should parse necessary information from
/// the http request, pass the data to a specified API servicer to perform the
/// required actions, then write the service results to the http response.
#[derive(Clone, Debug)]
pub struct ApiRouter<Api, NodeCaller> {
    /// The API internals it has.
    pub api: Api,
    /// An `Asserter` instance for the API.
    pub asserter: Asserter,
    /// The `NodeCaller` instance for the router.
    pub node_caller: NodeCaller,
}

impl<Api, NodeCaller> ApiRouter<Api, NodeCaller> {
    /// Creates a new ApiRouter from the given args.
    pub fn from<R: From<ApiRouter<Api, NodeCaller>>>(
        api: Api,
        asserter: Asserter,
        node_caller: NodeCaller,
    ) -> R {
        Self {
            api,
            asserter,
            node_caller,
        }
        .into()
    }
}

#[macro_export]
/// Creates a Router Specific type and implements From<ApiRouter> for it.
macro_rules! router {
    ($router:ident, $api:ident) => {
        #[derive(Clone, Debug)]
        pub struct $router<Api: $api> {
            /// The API internals it has.
            pub api: Api,
            /// An `Asserter` instance for the API.
            pub asserter: Asserter,
            /// The `NodeCaller` instance for the router.
            pub node_caller: Api::NodeCaller,
        }

        impl<Api: $api<NodeCaller = NodeCaller>, NodeCaller> From<ApiRouter<Api, NodeCaller>>
            for $router<Api>
        {
            fn from(router: ApiRouter<Api, NodeCaller>) -> Self {
                Self {
                    api: router.api,
                    asserter: router.asserter,
                    node_caller: router.node_caller,
                }
            }
        }
    };
}

/// ToRouter
pub trait ToRouter {
    /// Converts it to a router
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>>;
}
