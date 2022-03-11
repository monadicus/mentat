use mentat::{
    api::{CallApi, CallerCallApi},
    async_trait,
};

#[derive(Default)]
pub struct SnarkosCallApi;

#[async_trait]
impl CallerCallApi for SnarkosCallApi {}

#[async_trait]
impl CallApi for SnarkosCallApi {}
