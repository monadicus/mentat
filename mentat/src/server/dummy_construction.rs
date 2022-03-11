use super::*;

pub struct DummyConstructionApi;

#[axum::async_trait]
impl CallerConstructionApi for DummyConstructionApi {}

#[axum::async_trait]
impl ConstructionApi for DummyConstructionApi {}
