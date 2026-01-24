mod app_state;
mod auth;
mod bootstrap;
mod error;
mod response;
mod settings;

use http::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use axum::{
    Router,
    routing::{get, post},
};
use tokio::net;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::handlers::git_http::create_git_http_router;
use crate::handlers::organization::create_organization_router;
use crate::handlers::repository::{
    get_repository_commit_diffs, get_repository_commit_stats, get_repository_commits,
    get_repository_file, get_repository_file_commits, get_repository_tree,
};
use crate::handlers::repository_handlers::create_repository;

pub use app_state::AppState;
pub use auth::AuthenticatedUser;
pub use error::AppError;
pub use response::AppResponse;
pub use settings::Settings;

pub struct GitdotServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl GitdotServer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Arc::new(Settings::new()?);
        let pool = PgPool::connect(&settings.database_url).await?;
        let state = AppState::new(settings.clone(), pool);
        let router = create_router(state);
        let listener = tokio::net::TcpListener::bind(&settings.get_server_address())
            .await
            .unwrap();

        Ok(Self { router, listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!("Starting server on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("Failed to start server")?;
        Ok(())
    }
}

pub fn create_router(app_state: AppState) -> Router {
    let git_router = create_git_http_router();
    let org_router = create_organization_router();

    let repo_router = Router::new().route("/repository/{owner}/{repo}", post(create_repository));

    let old_repo_router = Router::new()
        .route("/repository/{owner}/{repo}/tree", get(get_repository_tree))
        .route("/repository/{owner}/{repo}/file", get(get_repository_file))
        .route(
            "/repository/{owner}/{repo}/commits",
            get(get_repository_commits),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}/stats",
            get(get_repository_commit_stats),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}/diffs",
            get(get_repository_commit_diffs),
        )
        .route(
            "/repository/{owner}/{repo}/file/commits",
            get(get_repository_file_commits),
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
        .merge(org_router)
        .merge(repo_router)
        .merge(old_repo_router)
        .layer(middleware)
        .with_state(app_state)
}
