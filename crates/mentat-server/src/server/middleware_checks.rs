//! This modules contains the middleware fn that performs all middleware checks.

use axum::{body::HttpBody, http::Extensions, middleware::Next, response::IntoResponse};
use hyper::{Body, Request};
use mentat_types::{MapErrMentat, NetworkIdentifier, Result};
use serde_json::Value;

use super::ServerType;
use crate::conf::{Configuration, Network, NodeConf};

// TODO i think this might be broken since its input is now an option, but it
// doesnt actually cause any errors
/// A function to do all middleware checks.
pub async fn middleware_checks<Types: ServerType>(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse> {
    let (parts, body) = req.into_parts();

    let req = if matches!(body.size_hint().exact(), Some(s) if s != 0) {
        let extensions = &parts.extensions;
        let bytes = hyper::body::to_bytes(body).await?;
        let json = serde_json::from_slice::<Value>(&bytes).merr(|e| e)?;
        Types::middleware_checks(extensions, &json)?;
        Request::from_parts(parts, Body::from(bytes))
    } else {
        Request::from_parts(parts, body)
    };

    Ok(next.run(req).await)
}

/// A struct for the checking the NetworkIdentifier.
pub struct NetworkIdentifierCheck;

impl NetworkIdentifierCheck {
    // TODO i think this might be broken since its input is now an option, but it
    // doesnt actually cause any errors
    /// A function to check if the server Blockchain specified matches the user
    /// request specified blockchain.
    pub fn check<Types: ServerType>(extensions: &Extensions, json: &Value) -> Result<()> {
        let config = extensions
            .get::<Configuration<Types::CustomConfig>>()
            .unwrap();
        if let Some(net_id) = json.get("network_identifier") {
            let network_identifier = serde_json::from_value::<NetworkIdentifier>(net_id.clone())?;
            if network_identifier.blockchain.to_uppercase()
                != Types::CustomConfig::BLOCKCHAIN.to_uppercase()
            {
                return Err(format!(
                    "invalid blockchain ID: found `{}`, expected `{}`",
                    network_identifier.blockchain.to_uppercase(),
                    Types::CustomConfig::BLOCKCHAIN.to_uppercase()
                )
                .into());
            } else if Network::from(network_identifier.network.to_uppercase()) != config.network {
                return Err(format!(
                    "invalid network ID: found `{}`, expected `{}`",
                    network_identifier.network.to_uppercase(),
                    config.network
                )
                .into());
            }
        }
        Ok(())
    }
}
