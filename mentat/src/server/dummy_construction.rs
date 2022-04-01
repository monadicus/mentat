use super::*;

#[derive(Clone, Default)]
pub struct DummyConstructionApi;

#[axum::async_trait]
impl CallerConstructionApi for DummyConstructionApi {}

#[axum::async_trait]
impl ConstructionApi for DummyConstructionApi {}
