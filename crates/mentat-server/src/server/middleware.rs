//! This modules contains the middleware fn that performs all middleware checks.

use axum::{middleware::Next, response::IntoResponse};
use hyper::{Body, HeaderMap, Method, Request, StatusCode};
use mentat_types::Result;

/// sets the `Content-Type` field in the response header to `application/json;
/// charset=UTF-8`
pub(crate) async fn content_type_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse> {
    let mut resp = next.run(req).await;
    resp.headers_mut().insert(
        "Content-Type",
        "application/json; charset=UTF-8".parse().unwrap(),
    );
    Ok(resp)
}

/// cors_middleware handles CORS and ensures OPTIONS requests are
/// handled properly.
///
/// This may be used to expose a Rosetta server instance to requests made by web
/// apps served over a different domain. Note that his currently allows _all_
/// third party domains so callers might want to adapt this middleware for their
/// own use-cases.
pub async fn cors_middleware(req: Request<Body>, next: Next<Body>) -> Result<impl IntoResponse> {
    let is_method = req.method() == Method::OPTIONS;
    let mut resp = next.run(req).await;

    let mut cors = HeaderMap::new();
    cors.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    cors.insert(
        "Access-Control-Allow-Headers",
        "Origin, X-Requested-With, Content-Type, Accept"
            .parse()
            .unwrap(),
    );
    cors.insert(
        "Access-Control-Allow-Methods",
        "GET, POST, OPTIONS".parse().unwrap(),
    );

    resp.headers_mut().extend(cors);
    if is_method {
        *resp.status_mut() = StatusCode::OK;
    }
    Ok(resp)
}
