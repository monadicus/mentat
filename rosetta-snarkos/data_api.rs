use mentat::api::{CallerDataApi, DataApi};

#[derive(Default)]
pub struct SnarkosDataApi;

#[rocket::async_trait]
impl CallerDataApi for SnarkosDataApi {}

#[rocket::async_trait]
impl DataApi for SnarkosDataApi {}
