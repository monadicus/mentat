#[macro_export]
macro_rules! jsonrpc_call {
    (@ret $method:expr, $params:expr, $rpc_caller:expr, $resp:ty) => {{
        let req = SnarkosJrpc::new($method, $params);

        let response = $rpc_caller
            .client
            .post(&$rpc_caller.node_rpc_url)
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
    (@res $method:expr, $params:expr, $rpc_caller:expr, $resp:ty) => {{
        let req = SnarkosJrpc::new($method, $params);

        let response = $rpc_caller
            .client
            .post(&$rpc_caller.node_rpc_url)
            .json(&req)
            .send()
            .await?;

        let snarkos_json: Response<$resp> = response.json().await?;
        match snarkos_json {
            Response::Ok(inner) => inner,
            Response::Err(err) => return err.into(),
        }
    }};
}
