use axum::{middleware::Next, response::IntoResponse};
use hyper::{Body, Request};
use serde_json::Value;

use super::ServerBuilder;
use crate::{
    errors::{MentatError, Result},
    identifiers::NetworkIdentifier,
};

pub async fn middleware_checks<Types: ServerBuilder>(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse> {
    let (parts, body) = req.into_parts();
    let extensions = &parts.extensions;
    let bytes = hyper::body::to_bytes(body).await?;
    let json = serde_json::from_slice::<Value>(&bytes).map_err(MentatError::from)?;

    NetworkIdentifier::check::<Types>(extensions, &json).await?;

    let req = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(req).await)
}
