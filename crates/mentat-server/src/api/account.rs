//! Houses the traits for the Rosetta Account API.
//! These traits are easily overridable for custom
//! implementations.

use std::fmt::Debug;

use axum::extract::{ConnectInfo, State};

use super::*;
use crate::conf::Configuration;

/// AccountAPIServicer defines the api actions for the AccountAPI service
#[axum::async_trait]
pub trait AccountApi: Clone + Debug + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

    /// Get an array of all AccountBalances for an
    /// [`crate::identifiers::AccountIdentifier`] and the
    /// [`crate::identifiers::BlockIdentifier`] at which the balance lookup was
    /// performed. The [`crate::identifiers::BlockIdentifier`] must always
    /// be returned because some consumers of account balance __data need to
    /// know specifically at which block the balance was calculated to
    /// compare balances they compute from operations with the balance
    /// returned by the node. It is important to note that making a balance
    /// request for an account without populating the [`crate::identifiers::
    /// SubAccountIdentifier`] should not result in the balance of all
    /// possible [`crate::identifiers::SubAccountIdentifier`]s being returned.
    /// Rather, it should result in the balance pertaining to no
    /// [`crate::identifiers::SubAccountIdentifier`]s being returned
    /// (sometimes called the liquid balance). To get all
    /// balances associated with an account, it may be necessary to perform
    /// multiple balance requests with unique
    /// [`crate::identifiers::AccountIdentifier`]s. It is also possible to
    /// perform a historical balance lookup (if the server supports it) by
    /// passing in an optional [`crate::identifiers::BlockIdentifier`].
    async fn account_balance(
        &self,
        _caller: Caller,
        _data: AccountBalanceRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<AccountBalanceResponse> {
        MentatError::not_implemented()
    }

    /// Get an array of all unspent coins for an
    /// [`crate::identifiers::AccountIdentifier`] and the
    /// [`crate::identifiers::BlockIdentifier`] at which the lookup was
    /// performed. If your implementation does not support coins (i.e. it is
    /// for an account-based blockchain), you do not need to implement this
    /// endpoint. If you implementation does support coins (i.e. it is fro a
    /// UTXO-based blockchain), you MUST also complete the /account/balance
    /// endpoint. It is important to note that making a coins request for an
    /// account without populating the
    /// [`crate::identifiers::SubAccountIdentifier`] should not result in the
    /// coins of all possible [`crate::identifiers::SubAccountIdentifier`]s
    /// being returned. Rather, it should result in the coins pertaining to
    /// no [`crate::identifiers::SubAccountIdentifier`]s being returned. To
    /// get all coins associated with an account, it may be necessary to
    /// perform multiple coin requests with unique
    /// [`crate::identifiers::AccountIdentifier]`s. Optionally, an
    /// implementation may choose to support updating an
    /// [`crate::identifiers::AccountIdentifier`]'s unspent coins based on the
    /// contents of the mempool. Note, using this functionality breaks any
    /// guarantee of idempotency.
    async fn account_coins(
        &self,
        _caller: Caller,
        _data: AccountCoinsRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<AccountCoinsResponse> {
        MentatError::not_implemented()
    }
}

/// AccountAPIRouter defines the required methods for binding the api requests
/// to a responses for the AccountAPI
/// The AccountAPIRouter implementation should parse necessary information from
/// the http request, pass the data to a AccountAPIServicer to perform the
/// required actions, then write the service results to the http response.
#[derive(Clone, Debug)]
pub struct AccountApiRouter<Api: AccountApi> {
    /// Api
    pub api: Api,
    /// Asserter
    pub asserter: Asserter,
    /// Caller
    pub node_caller: Api::NodeCaller,
}

impl<Api: AccountApi> AccountApiRouter<Api> {
    /// This endpoint only runs in online mode.
    #[tracing::instrument(name = "account_balance")]
    async fn call_account_balance(
        &self,
        caller: Caller,
        mode: &Mode,
        data: Option<UncheckedAccountBalanceRequest>,
    ) -> MentatResponse<UncheckedAccountBalanceResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            self.asserter.account_balance_request(data.as_ref())?;
            let resp = self
                .api
                .account_balance(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_account_coins(
        &self,
        caller: Caller,
        mode: &Mode,
        asserter: &Asserter,
        data: Option<UncheckedAccountCoinsRequest>,
    ) -> MentatResponse<UncheckedAccountCoinsResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
        } else {
            asserter.account_coins_request(data.as_ref())?;
            let resp = self
                .api
                .account_coins(caller, data.unwrap().into(), &self.node_caller)
                .await?
                .into();
            Ok(Json(resp))
        }
    }
}

impl<Api: AccountApi + 'static> ToRouter for AccountApiRouter<Api> {
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        axum::Router::new().route(
            "/balance",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 State(conf): State<Configuration<CustomConfig>>,
                 Json(req_data): Json<Option<UncheckedAccountBalanceRequest>>| async move {
                    self.call_account_balance(Caller { ip }, &conf.mode, req_data)
                        .await
                },
            ),
        )
    }
}
