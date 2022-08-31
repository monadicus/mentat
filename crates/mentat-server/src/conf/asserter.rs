//! TODO

#![allow(clippy::missing_docs_in_private_items)]

use mentat_asserter::Asserter;

macro_rules! builder_fn {
    ($name:ident) => {
        pub fn $name(mut self, v: Asserter) -> Self {
            self.$name = Some(v);
            self
        }
    };
}

#[derive(Default)]
pub struct AsserterTableBuilder {
    use_default: Option<Asserter>,
    account_api: Option<Asserter>,
    block_api: Option<Asserter>,
    call_api: Option<Asserter>,
    construction_api: Option<Asserter>,
    events_api: Option<Asserter>,
    mempool_api: Option<Asserter>,
    network_api: Option<Asserter>,
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
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AsserterTable {
    pub account_api: Asserter,
    pub block_api: Asserter,
    pub call_api: Asserter,
    pub construction_api: Asserter,
    pub events_api: Asserter,
    pub mempool_api: Asserter,
    pub network_api: Asserter,
    pub search_api: Asserter,
}

impl AsserterTable {
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
        }
    }
}
