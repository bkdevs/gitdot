mod git_info_refs;
mod git_receive_pack;
mod git_upload_pack;

use axum::{
    Router,
    body::Body,
    extract::DefaultBodyLimit,
    http::{Request, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
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
