//! This module contains a cli client for making Rosetta calls.

use core::fmt;

use anyhow::anyhow;
use mentat_types::*;
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

/// The client struct to call a rosetta API.
pub struct Client {
    /// The actual request client to do so.
    inner: reqwest::Client,
    /// The URL of the rosetta API being called.
    origin: Url,
}

/// The different types of Errors that can happen when using the CLI.
#[derive(Debug)]
pub enum ClientError {
    /// A CLI parsing error.
    ParseError(anyhow::Error),
    /// A networking error.
    NetworkError(anyhow::Error),
    /// A rosetta API error.
    ServerError(MentatError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::ServerError(e) => e.fmt(f),
            ClientError::NetworkError(e) => e.fmt(f),
            ClientError::ParseError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ClientError {}

/// The result type for the CLI.
type Result<T, E = ClientError> = std::result::Result<T, E>;

impl Client {
    /// `origin` should be of the form `http[s]://hostname:port/`
    pub fn new(origin: &str) -> anyhow::Result<Self> {
        Ok(Self::new_full(
            // ensure origin parses into a url
            origin.parse::<Url>()?,
            reqwest::ClientBuilder::default()
                .build()
                .map_err(|e| anyhow!(e))?,
        ))
    }

    /// `origin` should be of the form `http[s]://hostname:port/`.
    pub fn new_full(origin: Url, inner: reqwest::Client) -> Self {
        Self { inner, origin }
    }

    /// Create a post request AP.
    async fn post<Q: Serialize, R: DeserializeOwned>(&self, path: &str, request: &Q) -> Result<R> {
        let url = match self.origin.join(path) {
            Ok(url) => url.to_string(),
            Err(e) => return Err(ClientError::ParseError(anyhow!(e))),
        };
        let out = self.inner.post(url).json(request).send().await;
        match out {
            Err(e) => Err(ClientError::NetworkError(anyhow!(e))),
            Ok(response) => {
                if response.status().is_success() {
                    let out: R = match response.json().await {
                        Ok(x) => x,
                        Err(e) => return Err(ClientError::NetworkError(anyhow!(e))),
                    };
                    Ok(out)
                } else {
                    match response.json().await {
                        Ok(e) => Err(ClientError::ServerError(e)),
                        Err(e) => Err(ClientError::NetworkError(anyhow!(e))),
                    }
                }
            }
        }
    }

    /// Make a call to the /network/list Rosetta API endpoint.
    pub async fn network_list(
        &self,
        request: NullableMetadataRequest,
    ) -> Result<NullableNetworkListResponse> {
        let resp: NullableNetworkListResponse = self.post("network/list", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /network/options Rosetta API endpoint.
    pub async fn network_options(
        &self,
        request: NullableNetworkRequest,
    ) -> Result<NullableNetworkOptionsResponse> {
        let resp: NullableNetworkOptionsResponse = self.post("network/options", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /network/status Rosetta API endpoint.
    pub async fn network_status(
        &self,
        request: NullableNetworkRequest,
    ) -> Result<NullableNetworkStatusResponse> {
        let resp: NullableNetworkStatusResponse = self.post("network/status", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /account/balance Rosetta API endpoint.
    pub async fn account_balance(
        &self,
        request: NullableAccountBalanceRequest,
    ) -> Result<NullableAccountBalanceResponse> {
        let resp: NullableAccountBalanceResponse = self.post("account/balance", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /account/coins Rosetta API endpoint.
    pub async fn account_coins(
        &self,
        request: NullableAccountCoinsRequest,
    ) -> Result<NullableAccountCoinsResponse> {
        let resp: NullableAccountCoinsResponse = self.post("account/coins", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /block Rosetta API endpoint.
    pub async fn block(&self, request: NullableBlockRequest) -> Result<NullableBlockResponse> {
        let resp: NullableBlockResponse = self.post("block", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /block/transaction Rosetta API endpoint.
    pub async fn block_transaction(
        &self,
        request: NullableBlockTransactionRequest,
    ) -> Result<NullableBlockTransactionResponse> {
        let resp: NullableBlockTransactionResponse =
            self.post("block/transaction", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /mempool Rosetta API endpoint.
    pub async fn mempool(
        &self,
        request: NullableNetworkRequest,
    ) -> Result<NullableMempoolResponse> {
        let resp: NullableMempoolResponse = self.post("mempool", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /mempool/transaction Rosetta API endpoint.
    pub async fn mempool_transaction(
        &self,
        request: NullableMempoolTransactionRequest,
    ) -> Result<NullableMempoolTransactionResponse> {
        let resp: NullableMempoolTransactionResponse =
            self.post("mempool/transaction", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/combine Rosetta API endpoint.
    pub async fn construction_combine(
        &self,
        request: NullableConstructionCombineRequest,
    ) -> Result<NullableConstructionCombineResponse> {
        let resp: NullableConstructionCombineResponse =
            self.post("construction/combine", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/derive Rosetta API endpoint.
    pub async fn construction_derive(
        &self,
        request: NullableConstructionDeriveRequest,
    ) -> Result<NullableConstructionDeriveResponse> {
        let resp: NullableConstructionDeriveResponse =
            self.post("construction/derive", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/hash Rosetta API endpoint.
    pub async fn construction_hash(
        &self,
        request: NullableConstructionHashRequest,
    ) -> Result<NullableTransactionIdentifierResponse> {
        let resp: NullableTransactionIdentifierResponse =
            self.post("construction/hash", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/metadata Rosetta API endpoint.
    pub async fn construction_metadata(
        &self,
        request: NullableConstructionMetadataRequest,
    ) -> Result<ConstructionMetadataResponse> {
        let resp: NullableConstructionMetadataResponse =
            self.post("construction/metadata", &request).await?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/parse Rosetta API endpoint.
    pub async fn construction_parse(
        &self,
        request: NullableConstructionParseRequest,
    ) -> Result<NullableConstructionParseResponse> {
        let resp: NullableConstructionParseResponse =
            self.post("construction/parse", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/payloads Rosetta API endpoint.
    pub async fn construction_payloads(
        &self,
        request: NullableConstructionPayloadsRequest,
    ) -> Result<NullableConstructionPayloadsResponse> {
        let resp: NullableConstructionPayloadsResponse =
            self.post("construction/payloads", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/preprocess Rosetta API endpoint.
    pub async fn construction_preprocess(
        &self,
        request: NullableConstructionPreprocessRequest,
    ) -> Result<NullableConstructionPreprocessResponse> {
        let resp: NullableConstructionPreprocessResponse =
            self.post("construction/preprocess", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /construction/submit Rosetta API endpoint.
    pub async fn construction_submit(
        &self,
        request: NullableConstructionSubmitRequest,
    ) -> Result<NullableTransactionIdentifierResponse> {
        let resp: NullableTransactionIdentifierResponse =
            self.post("construction/submit", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /events/blocks Rosetta API endpoint.
    pub async fn events_blocks(
        &self,
        request: NullableEventsBlocksRequest,
    ) -> Result<NullableEventsBlocksResponse> {
        let resp: NullableEventsBlocksResponse = self.post("events/blocks", &request).await?;
        Ok(resp)
    }

    /// Make a call to the /search/transactions Rosetta API endpoint.
    pub async fn search_transactions(
        &self,
        request: NullableSearchTransactionsRequest,
    ) -> Result<NullableSearchTransactionsResponse> {
        let resp: NullableSearchTransactionsResponse =
            self.post("search/transactions", &request).await?;
        Ok(resp)
    }
}
