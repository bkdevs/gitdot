mod git_info_refs;
mod git_receive_pack;
mod git_upload_pack;

use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use futures::TryStreamExt;
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;
use tower_http::{
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::app::AppState;

use crate::util::LimitedReader;

use git_info_refs::git_info_refs;
use git_receive_pack::git_receive_pack;
use git_upload_pack::git_upload_pack;

const GIT_UPLOAD_PACK_BODY_LIMIT: usize = 10 * 1024 * 1024;
const GIT_RECEIVE_PACK_BODY_LIMIT: usize = 2 * 1024 * 1024 * 1024;
const GIT_UPLOAD_PACK_DECOMPRESSED_LIMIT: u64 = 100 * 1024 * 1024;
const GIT_RECEIVE_PACK_DECOMPRESSED_LIMIT: u64 = 4 * 1024 * 1024 * 1024;

pub fn create_git_http_router() -> Router<AppState> {
    Router::new()
        .route("/{owner}/{repo}/info/refs", get(git_info_refs))
        .route(
            "/{owner}/{repo}/git-upload-pack",
            post(git_upload_pack).layer(RequestBodyLimitLayer::new(GIT_UPLOAD_PACK_BODY_LIMIT)),
        )
        .route(
            "/{owner}/{repo}/git-receive-pack",
            post(git_receive_pack).layer(RequestBodyLimitLayer::new(GIT_RECEIVE_PACK_BODY_LIMIT)),
        )
        .layer(middleware::from_fn(add_www_authenticate_header))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::x_request_id())
}

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

async fn create_body_reader(
    headers: &HeaderMap,
    body: Body,
    max_decompressed: u64,
) -> Box<dyn AsyncRead + Unpin + Send> {
    let is_gzip = headers
        .get(header::CONTENT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.eq_ignore_ascii_case("gzip"));

    let stream = StreamReader::new(
        body.into_data_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
    );

    if !is_gzip {
        return Box::new(stream);
    }

    let buffered = tokio::io::BufReader::new(stream);
    let decoder = async_compression::tokio::bufread::GzipDecoder::new(buffered);
    Box::new(LimitedReader::new(decoder, max_decompressed))
}
