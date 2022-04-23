//! This modules contains the `RpcCaller` for mentat.

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    conf::{Configuration, NodeConf},
    errors::Result,
};

/// The `RpcCaller` struct is a wrapper to hold a rpc caller instance
/// that holds a request client and the url for the RPC.
#[derive(Clone, Debug)]
pub struct RpcCaller {
    /// The request client.
    pub client: reqwest::Client,
    /// The RPC url.
    pub node_rpc_url: reqwest::Url,
}

pub trait RpcResponse<O: DeserializeOwned>: DeserializeOwned {
    fn unwrap_response(self) -> Result<O>;
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

    pub async fn rpc_call<B: Serialize, O: DeserializeOwned, R: RpcResponse<O>>(
        &self,
        req: B,
    ) -> Result<O> {
        let resp = self
            .client
            .post(self.node_rpc_url.clone())
            .json(&req)
            .send()
            .await?;

        let resp_text = resp.text().await?;
        let response_type: R = serde_json::from_str::<R>(&resp_text)?;
        response_type.unwrap_response()
    }
}
