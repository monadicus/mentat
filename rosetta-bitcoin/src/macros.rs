#[macro_export]
macro_rules! jsonrpc_call {
    ($method:expr, $params:expr, $client:expr, $resp:ty) => {{
        let req = BitcoinJrpc::new($method, $params);

        let response = $client
            .client
            .post(&$client.node_rpc_url)
            .json(&req)
            .send()
            .await?;

        let bitcoin_text = response.text().await?;
        match serde_json::from_str::<Response<$resp>>(&bitcoin_text) {
            Ok(Response {
                result: Some(res),
                error: None,
            }) => res,
            Ok(Response {
                result: None,
                error: Some(err),
            }) => {
                return err.into();
            }
            Err(e) => Err(format!("error decoding: {}: {}", e, bitcoin_text))?,
            _ => Err(format!(
                "error decoding: expected either result or error: {}",
                bitcoin_text
            ))?,
        }
    }};
}
