use super::*;

#[derive(Default)]
pub struct DummyDataApi;

#[axum::async_trait]
impl CallerDataApi for DummyDataApi {}

#[axum::async_trait]
impl DataApi for DummyDataApi {}
