use super::*;

pub struct DummyDataApi;

#[async_trait::async_trait]
impl DataApi for DummyDataApi {}
