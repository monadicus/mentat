//! contains tools to supply custom asserters to each route group
use mentat_asserter::Asserter;

/// helper fn to construct builder methods
macro_rules! builder_fn {
    ($name:ident) => {
        pub fn $name(mut self, v: Asserter) -> Self {
            self.$name = Some(v);
            self
        }
    };
}

/// a builder struct that allows you to define custom asserters for different
/// route groups and a default asserter to fall back on if needed
#[derive(Default)]
pub struct AsserterTableBuilder {
    /// the default asserter to use for a route group if one wasn't set
    use_default: Option<Asserter>,
    /// the asserter for the routes in AccountApi
    account_api: Option<Asserter>,
    /// the asserter for the routes in BlockApi
    block_api: Option<Asserter>,
    /// the asserter for the routes in CallApi
    call_api: Option<Asserter>,
    /// the asserter for the routes in ConstructionApi
    construction_api: Option<Asserter>,
    /// the asserter for the routes in EventsApi
    events_api: Option<Asserter>,
    /// the asserter for the routes in MempoolApi
    mempool_api: Option<Asserter>,
    /// the asserter for the routes in NetworkApi
    network_api: Option<Asserter>,
    /// the asserter for the routes in SearchApi
    search_api: Option<Asserter>,
}

impl AsserterTableBuilder {
    builder_fn!(use_default);

    builder_fn!(account_api);

    builder_fn!(block_api);

    builder_fn!(call_api);

    builder_fn!(construction_api);

    builder_fn!(events_api);

    builder_fn!(mempool_api);

    builder_fn!(network_api);

    builder_fn!(search_api);

    /// constructs an AsserterTable from the builder. if any routes were left
    /// out then it will fall back to the default route. if no default route was
    /// provided then it will panic
    pub fn build(self) -> AsserterTable {
        AsserterTable {
            account_api: self.account_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no account asserter provided")
            }),
            block_api: self.block_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no block asserter provided")
            }),
            call_api: self
                .call_api
                .unwrap_or_else(|| self.use_default.clone().expect("no call asserter provided")),
            construction_api: self.construction_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no construction asserter provided")
            }),
            events_api: self.events_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no events asserter provided")
            }),
            mempool_api: self.mempool_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no mempool asserter provided")
            }),
            network_api: self.network_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no network asserter provided")
            }),
            search_api: self.search_api.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no search asserter provided")
            }),
            optional_api: (),
        }
    }
}

/// contains asserters for each api group
#[derive(Clone, Debug, Default)]
pub struct AsserterTable {
    /// the asserter for the routes in AccountApi
    pub account_api: Asserter,
    /// the asserter for the routes in BlockApi
    pub block_api: Asserter,
    /// the asserter for the routes in CallApi
    pub call_api: Asserter,
    /// the asserter for the routes in ConstructionApi
    pub construction_api: Asserter,
    /// the asserter for the routes in EventsApi
    pub events_api: Asserter,
    /// the asserter for the routes in MempoolApi
    pub mempool_api: Asserter,
    /// the asserter for the routes in NetworkApi
    pub network_api: Asserter,
    /// the asserter for the routes in SearchApi
    pub search_api: Asserter,
    // TODO
    /// exists due to an edge case in our proc macro
    pub optional_api: (),
}

impl AsserterTable {
    /// creates a builder that can be used to construct an asserter table
    pub fn builder() -> AsserterTableBuilder {
        AsserterTableBuilder::default()
    }
}

impl From<Asserter> for AsserterTable {
    fn from(v: Asserter) -> Self {
        Self {
            account_api: v.clone(),
            block_api: v.clone(),
            call_api: v.clone(),
            construction_api: v.clone(),
            events_api: v.clone(),
            mempool_api: v.clone(),
            network_api: v.clone(),
            search_api: v,
            optional_api: (),
        }
    }
}
