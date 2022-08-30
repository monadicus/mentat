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
    account: Option<Asserter>,
    block: Option<Asserter>,
    call: Option<Asserter>,
    construction: Option<Asserter>,
    events: Option<Asserter>,
    mempool: Option<Asserter>,
    network: Option<Asserter>,
    search: Option<Asserter>,
}

impl AsserterTableBuilder {
    builder_fn!(use_default);
    builder_fn!(account);
    builder_fn!(block);
    builder_fn!(call);
    builder_fn!(construction);
    builder_fn!(events);
    builder_fn!(mempool);
    builder_fn!(network);
    builder_fn!(search);

    pub fn build(self) -> AsserterTable {
        AsserterTable {
            account: self.account.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no account asserter provided")
            }),
            block: self.block.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no block asserter provided")
            }),
            call: self
                .call
                .unwrap_or_else(|| self.use_default.clone().expect("no call asserter provided")),
            construction: self.construction.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no construction asserter provided")
            }),
            events: self.events.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no events asserter provided")
            }),
            mempool: self.mempool.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no mempool asserter provided")
            }),
            network: self.network.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no network asserter provided")
            }),
            search: self.search.unwrap_or_else(|| {
                self.use_default
                    .clone()
                    .expect("no search asserter provided")
            }),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AsserterTable {
    pub account: Asserter,
    pub block: Asserter,
    pub call: Asserter,
    pub construction: Asserter,
    pub events: Asserter,
    pub mempool: Asserter,
    pub network: Asserter,
    pub search: Asserter,
}

impl AsserterTable {
    pub fn builder() -> AsserterTableBuilder {
        AsserterTableBuilder::default()
    }
}

impl From<Asserter> for AsserterTable {
    fn from(v: Asserter) -> Self {
        Self {
            account: v.clone(),
            block: v.clone(),
            call: v.clone(),
            construction: v.clone(),
            events: v.clone(),
            mempool: v.clone(),
            network: v.clone(),
            search: v,
        }
    }
}
