use super::*;

pub struct DummyCallApi;

#[rocket::async_trait]
impl CallApi for DummyCallApi {}

#[rocket::async_trait]
impl CallerCallApi for DummyCallApi {}
