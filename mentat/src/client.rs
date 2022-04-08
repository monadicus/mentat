use core::fmt;

use anyhow::anyhow;
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

use crate::{errors::ApiError, requests::*, responses::*};

pub struct Client {
    inner: reqwest::Client,
    origin: Url,
}

#[derive(Debug)]
pub enum ClientError {
    ParseError(anyhow::Error),
    NetworkError(anyhow::Error),
    ServerError(ApiError),
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

    pub async fn network_list(&self, request: &MetadataRequest) -> Result<NetworkListResponse> {
        self.post("network/list", request).await
    }

    pub async fn network_options(
        &self,
        request: &NetworkRequest,
    ) -> Result<NetworkOptionsResponse> {
        self.post("network/options", request).await
    }

    pub async fn network_status(&self, request: &NetworkRequest) -> Result<NetworkStatusResponse> {
        self.post("network/status", request).await
    }

    pub async fn account_balance(
        &self,
        request: &AccountBalanceRequest,
    ) -> Result<AccountBalanceResponse> {
        self.post("account/balance", request).await
    }

    pub async fn account_coins(
        &self,
        request: &AccountCoinsRequest,
    ) -> Result<AccountCoinsResponse> {
        self.post("account/coins", request).await
    }

    pub async fn block(&self, request: &BlockRequest) -> Result<BlockResponse> {
        self.post("block", request).await
    }

    pub async fn block_transaction(
        &self,
        request: &BlockTransactionRequest,
    ) -> Result<BlockTransactionResponse> {
        self.post("block/transaction", request).await
    }

    pub async fn mempool(&self, request: &NetworkRequest) -> Result<MempoolResponse> {
        self.post("mempool", request).await
    }

    pub async fn mempool_transaction(
        &self,
        request: &MempoolTransactionRequest,
    ) -> Result<MempoolTransactionResponse> {
        self.post("mempool/transaction", request).await
    }

    pub async fn construction_combine(
        &self,
        request: &ConstructionCombineRequest,
    ) -> Result<ConstructionCombineResponse> {
        self.post("construction/combine", request).await
    }

    pub async fn construction_derive(
        &self,
        request: &ConstructionDeriveRequest,
    ) -> Result<ConstructionDeriveResponse> {
        self.post("construction/derive", request).await
    }

    pub async fn construction_hash(
        &self,
        request: &ConstructionHashRequest,
    ) -> Result<TransactionIdentifierResponse> {
        self.post("construction/hash", request).await
    }

    pub async fn construction_metadata(
        &self,
        request: &ConstructionMetadataRequest,
    ) -> Result<ConstructionMetadataResponse> {
        self.post("construction/metadata", request).await
    }

    pub async fn construction_parse(
        &self,
        request: &ConstructionParseRequest,
    ) -> Result<ConstructionParseResponse> {
        self.post("construction/parse", request).await
    }

    pub async fn construction_payloads(
        &self,
        request: &ConstructionPayloadsRequest,
    ) -> Result<ConstructionPayloadsResponse> {
        self.post("construction/payloads", request).await
    }

    pub async fn construction_preprocess(
        &self,
        request: &ConstructionPreprocessRequest,
    ) -> Result<ConstructionPreprocessResponse> {
        self.post("construction/preprocess", request).await
    }

    pub async fn construction_submit(
        &self,
        request: &ConstructionSubmitRequest,
    ) -> Result<TransactionIdentifierResponse> {
        self.post("construction/submit", request).await
    }

    pub async fn events_blocks(
        &self,
        request: &EventsBlocksRequest,
    ) -> Result<EventsBlocksResponse> {
        self.post("events/blocks", request).await
    }

    pub async fn search_transactions(
        &self,
        request: &SearchTransactionsRequest,
    ) -> Result<SearchTransactionsResponse> {
        self.post("search/transactions", request).await
    }
}
