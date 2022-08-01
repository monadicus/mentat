//! This modules contains the `RpcCaller` for mentat.

use mentat_types::{MapErrMentat, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::conf::{Configuration, NodeConf};

/// The `RpcCaller` struct is a wrapper to hold a rpc caller instance
/// that holds a request client and the url for the RPC.
#[derive(Clone, Debug)]
pub struct RpcCaller {
    /// The request client.
    pub client: reqwest::Client,
    /// The RPC url.
    pub node_rpc_url: reqwest::Url,
}

/// A trait for users to implement unwrapping their RPC response.
pub trait RpcResponse: DeserializeOwned {
    /// The input type.
    type I: Serialize;
    /// The output type.
    type O;
    /// A function unwrap the response to the output type.
    fn unwrap_response(self) -> Result<Self::O>;
}

impl RpcCaller {
    /// Creates a new instance of the `RpcCaller`.
    pub fn new<CustomConf: NodeConf + DeserializeOwned + Serialize>(
        conf: &Configuration<CustomConf>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            node_rpc_url: NodeConf::build_url(conf),
        }
    }

    /// Makes the RPC call returning the expected output given the input type.
    pub async fn rpc_call<R: RpcResponse>(&self, req: R::I) -> Result<R::O> {
        let resp = self
            .client
            .post(self.node_rpc_url.clone())
            .json(&req)
            .send()
            .await?;

        let resp_text = resp.text().await?;
        let response_type: R = serde_json::from_str::<R>(&resp_text)
            .merr(|e| format!("failed to serialize response: `{e}`\ntext: `{resp_text}`"))?;
        response_type.unwrap_response()
    }
}
