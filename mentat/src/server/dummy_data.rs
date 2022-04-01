use super::*;

#[derive(Clone, Default)]
pub struct DummyDataApi;

#[axum::async_trait]
impl CallerDataApi for DummyDataApi {}

#[axum::async_trait]
impl DataApi for DummyDataApi {}
