use mentat::{
    api::{CallApi, CallerCallApi},
    axum::async_trait,
};

#[derive(Clone, Default)]
pub struct SnarkosCallApi;

#[async_trait]
impl CallerCallApi for SnarkosCallApi {}

#[async_trait]
impl CallApi for SnarkosCallApi {}
