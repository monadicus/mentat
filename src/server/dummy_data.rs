use super::*;

pub struct DummyDataApi;

#[rocket::async_trait]
impl CallerDataApi for DummyDataApi {}

#[rocket::async_trait]
impl DataApi for DummyDataApi {}
