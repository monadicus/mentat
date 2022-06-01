//! This module contains a cli client for making Rosetta calls.

use core::fmt;

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use crate::{errors::MentatError, requests::*, responses::*};

/// The client struct to call a rosetta API.
#[wasm_bindgen]
pub struct Client {
    inner: RequestInit,
    origin: String,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type JSClientError =
    | NetworkError(string)
    | ServerError(ApiError);

"#;

/// The different types of Errors that can happen when using the CLI.
#[derive(Debug)]
pub enum ClientError {
    /// A networking error
    NetworkError(String),
    /// A rosetta API error.
    ServerError(MentatError),
}

impl From<JsValue> for ClientError {
    fn from(v: JsValue) -> Self {
        let is_api_error: Result<MentatError, _> = v.into_serde();
        match is_api_error {
            Ok(api_error) => ClientError::ServerError(api_error),
            Err(e) => ClientError::NetworkError(e.to_string()),
        }
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::NetworkError(err.to_string())
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::ServerError(e) => e.fmt(f),
            ClientError::NetworkError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ClientError {}

/// The result type for the CLI.
type Result<T, E = ClientError> = std::result::Result<T, E>;

#[wasm_bindgen]
impl Client {
    /// `origin` should be of the form `http[s]://hostname:port/`
    #[wasm_bindgen(constructor)]
    pub fn new(origin: &str) -> Self {
        let mut opts = RequestInit::new();
        opts.method("POST");
        Self::new_full(
            // ensure origin parses into a url
            origin.to_string(),
            opts,
        )
    }

    /// `origin` should be of the form `http[s]://hostname:port/`.
    #[wasm_bindgen]
    pub fn new_full(origin: String, inner: RequestInit) -> Self {
        Self { inner, origin }
    }

    /// Create a post request to a Rosetta API.
    async fn post<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        request_body: &Q,
    ) -> Result<R> {
        let url = format!("{}/{path}", self.origin);
        let mut request_init = self.inner.clone();
        request_init.body(Some(&JsValue::from_serde(request_body)?));
        let request = Request::new_with_str_and_init(&url, &self.inner)?;
        let window = web_sys::window().ok_or_else(|| {
            ClientError::NetworkError(
                "Error: Failed to get
    window."
                    .to_string(),
            )
        })?;
        let out = JsFuture::from(window.fetch_with_request(&request)).await?;
        let response: Response = out.dyn_into()?;
        if (200..=299).contains(&response.status()) {
            let out: R = match response.json() {
                Ok(x) => JsFuture::from(x).await?.into_serde()?,
                Err(e) => return Err(e.into()),
            };
            Ok(out)
        } else {
            match response.json() {
                Ok(e) => {
                    let api_error: MentatError = JsFuture::from(e).await?.into_serde()?;
                    Err(ClientError::ServerError(api_error))
                }
                Err(e) => Err(e.into()),
            }
        }
    }

    /// Make a call to the /network/list Rosetta API endpoint.
    #[wasm_bindgen]
    pub async fn network_list(self, request: MetadataRequest) -> NetworkListResponse {
        self.post("network/list", &request).await.unwrap()
    }

    /*
    /// Make a call to the /network/options Rosetta API endpoint.
    pub async fn network_options(
        &self,
        request: &NetworkRequest,
    ) -> Result<NetworkOptionsResponse> {
        self.post("network/options", request).await
    }

    /// Make a call to the /network/status Rosetta API endpoint.
    pub async fn network_status(&self, request: &NetworkRequest) -> Result<NetworkStatusResponse> {
        self.post("network/status", request).await
    }

    /// Make a call to the /account/balance Rosetta API endpoint.
    pub async fn account_balance(
        &self,
        request: &AccountBalanceRequest,
    ) -> Result<AccountBalanceResponse> {
        self.post("account/balance", request).await
    }

    /// Make a call to the /account/coins Rosetta API endpoint.
    pub async fn account_coins(
        &self,
        request: &AccountCoinsRequest,
    ) -> Result<AccountCoinsResponse> {
        self.post("account/coins", request).await
    }

    /// Make a call to the /block Rosetta API endpoint.
    pub async fn block(&self, request: &BlockRequest) -> Result<BlockResponse> {
        self.post("block", request).await
    }

    /// Make a call to the /block/transaction Rosetta API endpoint.
    pub async fn block_transaction(
        &self,
        request: &BlockTransactionRequest,
    ) -> Result<BlockTransactionResponse> {
        self.post("block/transaction", request).await
    }

    /// Make a call to the /mempool Rosetta API endpoint.
    pub async fn mempool(&self, request: &NetworkRequest) -> Result<MempoolResponse> {
        self.post("mempool", request).await
    }

    /// Make a call to the /mempool/transaction Rosetta API endpoint.
    pub async fn mempool_transaction(
        &self,
        request: &MempoolTransactionRequest,
    ) -> Result<MempoolTransactionResponse> {
        self.post("mempool/transaction", request).await
    }

    /// Make a call to the /construction/combine Rosetta API endpoint.
    pub async fn construction_combine(
        &self,
        request: &ConstructionCombineRequest,
    ) -> Result<ConstructionCombineResponse> {
        self.post("construction/combine", request).await
    }

    /// Make a call to the /construction/derive Rosetta API endpoint.
    pub async fn construction_derive(
        &self,
        request: &ConstructionDeriveRequest,
    ) -> Result<ConstructionDeriveResponse> {
        self.post("construction/derive", request).await
    }

    /// Make a call to the /construction/hash Rosetta API endpoint.
    pub async fn construction_hash(
        &self,
        request: &ConstructionHashRequest,
    ) -> Result<TransactionIdentifierResponse> {
        self.post("construction/hash", request).await
    }

    /// Make a call to the /construction/metadata Rosetta API endpoint.
    pub async fn construction_metadata(
        &self,
        request: &ConstructionMetadataRequest,
    ) -> Result<ConstructionMetadataResponse> {
        self.post("construction/metadata", request).await
    }

    /// Make a call to the /construction/parse Rosetta API endpoint.
    pub async fn construction_parse(
        &self,
        request: &ConstructionParseRequest,
    ) -> Result<ConstructionParseResponse> {
        self.post("construction/parse", request).await
    }

    /// Make a call to the /construction/payloads Rosetta API endpoint.
    pub async fn construction_payloads(
        &self,
        request: &ConstructionPayloadsRequest,
    ) -> Result<ConstructionPayloadsResponse> {
        self.post("construction/payloads", request).await
    }

    /// Make a call to the /construction/preprocess Rosetta API endpoint.
    pub async fn construction_preprocess(
        &self,
        request: &ConstructionPreprocessRequest,
    ) -> Result<ConstructionPreprocessResponse> {
        self.post("construction/preprocess", request).await
    }

    /// Make a call to the /construction/submit Rosetta API endpoint.
    pub async fn construction_submit(
        &self,
        request: &ConstructionSubmitRequest,
    ) -> Result<TransactionIdentifierResponse> {
        self.post("construction/submit", request).await
    }

    /// Make a call to the /events/blocks Rosetta API endpoint.
    pub async fn events_blocks(
        &self,
        request: &EventsBlocksRequest,
    ) -> Result<EventsBlocksResponse> {
        self.post("events/blocks", request).await
    }

    /// Make a call to the /search/transactions Rosetta API endpoint.
    pub async fn search_transactions(
        &self,
        request: &SearchTransactionsRequest,
    ) -> Result<SearchTransactionsResponse> {
        self.post("search/transactions", request).await
    } */
}
