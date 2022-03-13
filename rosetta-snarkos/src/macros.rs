#[macro_export]
macro_rules! jsonrpc_call {
    ($method:expr, $params:expr, $client:expr, $resp:ty) => {{
        let req = SnarkosJrpc::new($method, $params);

        let response = $client
            .post("http://127.0.0.1:3032")
            .json(&req)
            .send()
            .await?;

        let snarkos_json: Response<$resp> = response.json().await?;
        // tracing::debug!("{snarkos_json:?}");
        match snarkos_json {
            Response::Ok(inner) => inner.into(),
            Response::Err(err) => err.into(),
        }
    }};
}
