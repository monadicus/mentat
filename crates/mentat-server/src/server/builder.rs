//! Defines a builder pattern for a `Server`.

use std::sync::Arc;

use super::{types::*, Server};
use crate::{api::*, conf::*};

/// The Struct for building a `Server`.
pub struct ServerBuilder<Types: ServerType> {
    /// The Account API endpoints.
    account_api: Option<Types::AccountApi>,
    /// The Block API endpoints.
    block_api: Option<Types::BlockApi>,
    /// The Call API endpoints.
    call_api: Option<Types::CallApi>,
    /// The Construction API endpoints.
    construction_api: Option<Types::ConstructionApi>,
    /// The Events API endpoints.
    events_api: Option<Types::EventsApi>,
    /// The Mempool API endpoints.
    mempool_api: Option<Types::MempoolsApi>,
    /// The network API endpoints.
    network_api: Option<Types::NetworkApi>,
    /// The search API endpoints.
    search_api: Option<Types::SearchApi>,
    /// The Optional API endpoints.
    optional_api: Option<(Types::OptionalApi, bool)>,
    /// The Caller used to interact with the node
    node_caller: Option<Types::NodeCaller>,
    /// The optional configuration details.
    configuration: Option<Configuration<Types::CustomConfig>>,
}

impl<Types: ServerType> Default for ServerBuilder<Types> {
    fn default() -> Self {
        Self {
            account_api: None,
            block_api: None,
            call_api: None,
            construction_api: None,
            events_api: None,
            mempool_api: None,
            network_api: None,
            search_api: None,
            optional_api: None,
            node_caller: None,
            configuration: None,
        }
    }
}

impl<Types: ServerType> ServerBuilder<Types> {
    /// Builds the `Server`.
    pub fn build(self) -> Server<Types> {
        let configuration = self
            .configuration
            .expect("You did not set the custom configuration.");
        let asserters = Types::init_asserters(&configuration);
        let node_caller = Arc::new(self.node_caller.expect("You did not set the node caller"));
        Server {
            account_api: self
                .account_api
                .map(|api| ApiRouter::from(api, asserters.account_api, node_caller.clone()))
                .expect("You did not set the call api."),
            block_api: self
                .block_api
                .map(|api| ApiRouter::from(api, asserters.block_api, node_caller.clone()))
                .expect("You did not set the call api."),
            call_api: self
                .call_api
                .map(|api| ApiRouter::from(api, asserters.call_api, node_caller.clone()))
                .expect("You did not set the call api."),
            construction_api: self
                .construction_api
                .map(|api| ApiRouter::from(api, asserters.construction_api, node_caller.clone()))
                .expect("You did not set the construction api."),
            events_api: self
                .events_api
                .map(|api| ApiRouter::from(api, asserters.events_api, node_caller.clone()))
                .expect("You did not set the call api."),
            mempool_api: self
                .mempool_api
                .map(|api| ApiRouter::from(api, asserters.mempool_api, node_caller.clone()))
                .expect("You did not set the call api."),
            network_api: self
                .network_api
                .map(|api| ApiRouter::from(api, asserters.network_api, node_caller.clone()))
                .expect("You did not set the call api."),
            optional_api: self
                .optional_api
                .map(|(api, enabled)| OptionalApiRouter {
                    api,
                    enabled,
                    node_caller: node_caller.clone(),
                })
                .expect("You did not set the additional api."),
            search_api: self
                .search_api
                .map(|api| ApiRouter::from(api, asserters.search_api, node_caller.clone()))
                .expect("You did not set the call api."),

            configuration,
        }
    }

    /// Sets the Account API on the builder.
    pub fn account_api(mut self, a: Types::AccountApi) -> Self {
        self.account_api = Some(a);
        self
    }

    /// Sets the Block API on the builder.
    pub fn block_api(mut self, a: Types::BlockApi) -> Self {
        self.block_api = Some(a);
        self
    }

    /// Sets the Call API on the builder.
    pub fn call_api(mut self, a: Types::CallApi) -> Self {
        self.call_api = Some(a);
        self
    }

    /// Sets the construction API on the builder.
    pub fn construction_api(mut self, a: Types::ConstructionApi) -> Self {
        self.construction_api = Some(a);
        self
    }

    /// Sets the Events API on the builder.
    pub fn events_api(mut self, a: Types::EventsApi) -> Self {
        self.events_api = Some(a);
        self
    }

    /// Sets the Mempool API on the builder.
    pub fn mempool_api(mut self, a: Types::MempoolsApi) -> Self {
        self.mempool_api = Some(a);
        self
    }

    /// Sets the Network API on the builder.
    pub fn network_api(mut self, a: Types::NetworkApi) -> Self {
        self.network_api = Some(a);
        self
    }

    /// Sets the optional API on the builder.
    pub fn optional_api(mut self, a: Types::OptionalApi, enabled: bool) -> Self {
        self.optional_api = Some((a, enabled));
        self
    }

    /// Sets the Search API on the builder.
    pub fn search_api(mut self, a: Types::SearchApi) -> Self {
        self.search_api = Some(a);
        self
    }

    /// Sets the custom configuration from a cli arg on the builder.
    pub fn custom_configuration_from_arg(self) -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("Expected usage: <{}> <configuration file>", args[0]);
            std::process::exit(1);
        }

        let path = std::path::PathBuf::from(&args[1]);
        self.custom_configuration(&path)
    }

    /// Sets the custom configuration on the builder from a path and then sets
    /// the node caller generated from the config.
    pub fn custom_configuration(mut self, path: &std::path::Path) -> Self {
        let config = Configuration::load(path);
        self.node_caller = Some(config.clone().into());
        self.configuration = Some(config);
        self
    }
}
