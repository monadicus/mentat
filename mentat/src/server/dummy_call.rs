use super::*;

#[derive(Clone, Default)]
pub struct DummyCallApi;

#[axum::async_trait]
impl CallApi for DummyCallApi {}

#[axum::async_trait]
impl CallerCallApi for DummyCallApi {}
