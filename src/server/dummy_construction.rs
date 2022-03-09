use super::*;

pub struct DummyConstructionApi;

#[async_trait::async_trait]
impl ConstructionApi for DummyConstructionApi {}
