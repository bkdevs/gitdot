mod process_post_receive;

use std::{net::SocketAddr, time::Duration};

use axum::{
    Router,
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::app::AppState;

use process_post_receive::process_post_receive;

pub fn create_internal_router() -> Router<AppState> {
    Router::new()
        .route(
            "/internal/{owner}/{repo}/process-post-receive",
            post(process_post_receive),
        )
        .layer(middleware::from_fn(require_localhost))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(PropagateRequestIdLayer::x_request_id())
}

/// Middleware to require that the request is coming from localhost.
async fn require_localhost(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if !addr.ip().is_loopback() {
        return StatusCode::FORBIDDEN.into_response();
    }
    next.run(request).await
}
