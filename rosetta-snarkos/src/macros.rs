#[macro_export]
macro_rules! jsonrpc_call {
    ($method:expr, $params:expr, $rpc_caller:expr, $resp:ty) => {{
        let req = SnarkosJrpc::new($method, $params);

        let response = $rpc_caller
            .client
            .post(&$rpc_caller.node_rpc_url)
            .json(&req)
            .send()
            .await?;

        let snarkos_text = response.text().await?;
        // tracing::debug!("{snarkos_json:?}");
        match serde_json::from_str::<Response<$resp>>(&snarkos_text) {
            Ok(Response::Ok(res)) => res,
            Ok(Response::Err(err)) => {
                return err.into();
            }
            Err(e) => return Err(format!("error decoding: {}: {}", e, snarkos_text).into()),
        }
    }};
}
