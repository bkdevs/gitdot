use http::StatusCode;
use std::time::Duration;

use axum::{
    Router,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::handlers::git_smart_http::{git_info_refs, git_receive_pack, git_upload_pack};
use crate::handlers::repository::{
    create_repository, get_repository_commits, get_repository_file, get_repository_file_history,
    get_repository_tree,
};

use super::app_state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    let git_router = Router::new()
        .route("/{owner}/{repo}/info/refs", get(git_info_refs))
        .route("/{owner}/{repo}/git-upload-pack", post(git_upload_pack))
        .route("/{owner}/{repo}/git-receive-pack", post(git_receive_pack));

    let repo_router = Router::new()
        .route("/repository/{owner}/{repo}", post(create_repository))
        .route("/repository/{owner}/{repo}/tree", get(get_repository_tree))
        .route("/repository/{owner}/{repo}/file", get(get_repository_file))
        .route(
            "/repository/{owner}/{repo}/commits",
            get(get_repository_commits),
        )
        .route(
            "/repository/{owner}/{repo}/file/history",
            get(get_repository_file_history),
        );

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(PropagateRequestIdLayer::x_request_id());

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(git_router)
        .merge(repo_router)
        .layer(middleware)
        .with_state(app_state)
}
