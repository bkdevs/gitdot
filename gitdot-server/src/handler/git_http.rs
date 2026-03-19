mod git_info_refs;
mod git_receive_pack;
mod git_upload_pack;

use std::io::Read;

use axum::{
    Router,
    body::Body,
    extract::DefaultBodyLimit,
    http::{HeaderMap, Request, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use flate2::read::GzDecoder;
use futures::TryStreamExt;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_util::io::StreamReader;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::app::AppState;

use git_info_refs::git_info_refs;
use git_receive_pack::git_receive_pack;
use git_upload_pack::git_upload_pack;

pub fn create_git_http_router() -> Router<AppState> {
    Router::new()
        .route("/{owner}/{repo}/info/refs", get(git_info_refs))
        .route("/{owner}/{repo}/git-upload-pack", post(git_upload_pack))
        .route("/{owner}/{repo}/git-receive-pack", post(git_receive_pack))
        .layer(middleware::from_fn(add_www_authenticate_header))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(DefaultBodyLimit::disable())
}

/// Middleware to add the WWW-Authenticate header to the response as
/// git expects a WWW-Authenticate header when unauthorized.
async fn add_www_authenticate_header(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    if response.status() == StatusCode::UNAUTHORIZED {
        response.headers_mut().insert(
            header::WWW_AUTHENTICATE,
            "Basic realm=\"gitdot\"".parse().unwrap(),
        );
    }
    response
}

/// Converts an axum Body into an AsyncRead, decompressing gzip if Content-Encoding indicates it.
/// Git clients (e.g. `git clone --mirror`) may gzip-compress the request body.
async fn create_body_reader(headers: &HeaderMap, body: Body) -> Box<dyn AsyncRead + Unpin + Send> {
    let is_gzip = headers
        .get("content-encoding")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == "gzip");

    let mut reader: Box<dyn AsyncRead + Unpin + Send> = Box::new(StreamReader::new(
        body.into_data_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
    ));

    if !is_gzip {
        return reader;
    }

    let mut compressed = Vec::new();
    if let Err(e) = reader.read_to_end(&mut compressed).await {
        tracing::error!(error = %e, "failed to read gzip request body");
        return Box::new(std::io::Cursor::new(Vec::new()));
    }

    let mut decoder = GzDecoder::new(compressed.as_slice());
    let mut decompressed = Vec::new();
    if let Err(e) = decoder.read_to_end(&mut decompressed) {
        tracing::error!(error = %e, "failed to decompress gzip request body");
        return Box::new(std::io::Cursor::new(Vec::new()));
    }

    Box::new(std::io::Cursor::new(decompressed))
}
