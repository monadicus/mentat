//! This modules contains the middleware fn that performs all middleware checks.

use axum::{body::HttpBody, middleware::Next, response::IntoResponse};
use hyper::{Body, Request};
use serde_json::Value;

use super::ServerType;
use crate::errors::{MapErrMentat, Result};

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
