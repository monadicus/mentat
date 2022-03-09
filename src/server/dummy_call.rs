use super::*;

pub struct DummyCallApi;

#[async_trait::async_trait]
impl CallApi for DummyCallApi {}

#[async_trait::async_trait]
impl CallerCallApi for DummyCallApi {}
