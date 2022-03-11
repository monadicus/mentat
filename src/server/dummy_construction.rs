use super::*;

pub struct DummyConstructionApi;

#[rocket::async_trait]
impl CallerConstructionApi for DummyConstructionApi {}

#[rocket::async_trait]
impl ConstructionApi for DummyConstructionApi {}
