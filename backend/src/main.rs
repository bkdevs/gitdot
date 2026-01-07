mod config;
mod dto;
mod handlers;
mod utils;

use http::StatusCode;
use std::sync::Arc;
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

use crate::config::{app_state::AppState, settings::Settings};
use crate::handlers::git_smart_http::{git_info_refs, git_receive_pack, git_upload_pack};
use crate::handlers::repository::{
    create_repository, get_repository_commits, get_repository_file, get_repository_tree,
};

fn create_router(app_state: AppState) -> Router {
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
        );

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(git_router)
        .merge(repo_router)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(10),
                ))
                .layer(PropagateRequestIdLayer::x_request_id()),
        )
        .with_state(app_state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gitdot=debug,tower_http=debug".into()),
        )
        .init();

    let settings = Arc::new(Settings::new().expect("Failed to load settings"));
    let address = settings.get_server_address();
    let state = AppState::new(settings);
    tracing::info!("Starting server on {}", address);

    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
