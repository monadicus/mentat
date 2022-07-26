//! This module contains a cli client for making Rosetta calls.

use core::fmt;

use anyhow::anyhow;
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    asserter::{
        account_balance_response, account_coins, construction_combine_response,
        construction_derive_response, construction_metadata_response,
        construction_payloads_response, construction_preprocess_response, events_blocks_response,
        mempool_transactions, network_list_response, network_options_response,
        network_status_response, transaction_identifier_response, Asserter,
    },
    types::*,
};

/// The client struct to call a rosetta API.
pub struct Client {
    /// The actual request client to do so.
    inner: reqwest::Client,
    /// the response asserter
    asserter: Asserter,
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
        Self {
            inner,
            asserter: todo!(),
            origin,
        }
    }

    /// Create a post request to a Rosetta API.
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
    pub async fn network_list(&self, request: MetadataRequest) -> Result<NetworkListResponse> {
        let resp = self
            .post("network/list", &NullableMetadataRequest::from(request))
            .await?;
        network_list_response(Some(&resp)).map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /network/options Rosetta API endpoint.
    pub async fn network_options(&self, request: NetworkRequest) -> Result<NetworkOptionsResponse> {
        let resp = self
            .post("network/options", &NullableNetworkRequest::from(request))
            .await?;
        network_options_response(Some(&resp)).map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /network/status Rosetta API endpoint.
    pub async fn network_status(&self, request: NetworkRequest) -> Result<NetworkStatusResponse> {
        let resp = self
            .post("network/status", &NullableNetworkRequest::from(request))
            .await?;
        network_status_response(Some(&resp)).map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /account/balance Rosetta API endpoint.
    pub async fn account_balance(
        &self,
        request: AccountBalanceRequest,
    ) -> Result<AccountBalanceResponse> {
        let resp = self
            .post(
                "account/balance",
                &NullableAccountBalanceRequest::from(request),
            )
            .await?;
        account_balance_response(todo!(), &resp).map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /account/coins Rosetta API endpoint.
    pub async fn account_coins(
        &self,
        request: AccountCoinsRequest,
    ) -> Result<AccountCoinsResponse> {
        let resp = self
            .post("account/coins", &NullableAccountCoinsRequest::from(request))
            .await?;
        account_coins(&resp).map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /block Rosetta API endpoint.
    pub async fn block(&self, request: BlockRequest) -> Result<BlockResponse> {
        let resp: NullableBlockResponse = self
            .post("block", &NullableBlockRequest::from(request))
            .await?;
        self.asserter
            .block(resp.block.as_ref())
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /block/transaction Rosetta API endpoint.
    pub async fn block_transaction(
        &self,
        request: BlockTransactionRequest,
    ) -> Result<BlockTransactionResponse> {
        let resp: NullableBlockTransactionResponse = self
            .post(
                "block/transaction",
                &NullableBlockTransactionRequest::from(request),
            )
            .await?;
        self.asserter
            .transaction(resp.transaction.as_ref())
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /mempool Rosetta API endpoint.
    pub async fn mempool(&self, request: NetworkRequest) -> Result<MempoolResponse> {
        let resp: NullableMempoolResponse = self
            .post("mempool", &NullableNetworkRequest::from(request))
            .await?;
        mempool_transactions(&resp.transaction_identifiers)
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /mempool/transaction Rosetta API endpoint.
    pub async fn mempool_transaction(
        &self,
        request: MempoolTransactionRequest,
    ) -> Result<MempoolTransactionResponse> {
        let resp: NullableMempoolTransactionResponse = self
            .post(
                "mempool/transaction",
                &NullableMempoolTransactionRequest::from(request),
            )
            .await?;
        self.asserter
            .transaction(resp.transaction.as_ref())
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/combine Rosetta API endpoint.
    pub async fn construction_combine(
        &self,
        request: ConstructionCombineRequest,
    ) -> Result<ConstructionCombineResponse> {
        let resp = self
            .post(
                "construction/combine",
                &NullableConstructionCombineRequest::from(request),
            )
            .await?;
        construction_combine_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/derive Rosetta API endpoint.
    pub async fn construction_derive(
        &self,
        request: ConstructionDeriveRequest,
    ) -> Result<ConstructionDeriveResponse> {
        let resp = self
            .post(
                "construction/derive",
                &NullableConstructionDeriveRequest::from(request),
            )
            .await?;
        construction_derive_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/hash Rosetta API endpoint.
    pub async fn construction_hash(
        &self,
        request: ConstructionHashRequest,
    ) -> Result<TransactionIdentifierResponse> {
        let resp = self
            .post(
                "construction/hash",
                &NullableConstructionHashRequest::from(request),
            )
            .await?;
        transaction_identifier_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/metadata Rosetta API endpoint.
    pub async fn construction_metadata(
        &self,
        request: ConstructionMetadataRequest,
    ) -> Result<ConstructionMetadataResponse> {
        let resp = self
            .post(
                "construction/metadata",
                &NullableConstructionMetadataRequest::from(request),
            )
            .await?;
        construction_metadata_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/parse Rosetta API endpoint.
    pub async fn construction_parse(
        &self,
        request: ConstructionParseRequest,
    ) -> Result<ConstructionParseResponse> {
        let resp = self
            .post(
                "construction/parse",
                &NullableConstructionParseRequest::from(request),
            )
            .await?;
        self.asserter
            .construction_parse_response(Some(&resp), todo!())
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/payloads Rosetta API endpoint.
    pub async fn construction_payloads(
        &self,
        request: ConstructionPayloadsRequest,
    ) -> Result<ConstructionPayloadsResponse> {
        let resp = self
            .post(
                "construction/payloads",
                &NullableConstructionPayloadsRequest::from(request),
            )
            .await?;
        construction_payloads_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/preprocess Rosetta API endpoint.
    pub async fn construction_preprocess(
        &self,
        request: ConstructionPreprocessRequest,
    ) -> Result<ConstructionPreprocessResponse> {
        let resp = self
            .post(
                "construction/preprocess",
                &NullableConstructionPreprocessRequest::from(request),
            )
            .await?;
        construction_preprocess_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /construction/submit Rosetta API endpoint.
    pub async fn construction_submit(
        &self,
        request: ConstructionSubmitRequest,
    ) -> Result<TransactionIdentifierResponse> {
        let resp = self
            .post(
                "construction/submit",
                &NullableConstructionSubmitRequest::from(request),
            )
            .await?;
        transaction_identifier_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /events/blocks Rosetta API endpoint.
    pub async fn events_blocks(
        &self,
        request: EventsBlocksRequest,
    ) -> Result<EventsBlocksResponse> {
        let resp = self
            .post("events/blocks", &NullableEventsBlocksRequest::from(request))
            .await?;
        events_blocks_response(Some(&resp)).map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }

    /// Make a call to the /search/transactions Rosetta API endpoint.
    pub async fn search_transactions(
        &self,
        request: SearchTransactionsRequest,
    ) -> Result<SearchTransactionsResponse> {
        let resp = self
            .post(
                "search/transactions",
                &NullableSearchTransactionsRequest::from(request),
            )
            .await?;
        self.asserter
            .search_transaction_response(Some(&resp))
            .map_err(|e| ClientError::ServerError(e.into()))?;
        Ok(resp.into())
    }
}
