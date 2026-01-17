mod app_state;
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

use crate::handlers::git_smart_http::{git_info_refs, git_receive_pack, git_upload_pack};
use crate::handlers::organization_handlers::create_organization;
use crate::handlers::repository::{
    create_repository, get_repository_commit_diffs, get_repository_commits, get_repository_file,
    get_repository_file_commits, get_repository_tree,
};

pub use app_state::AppState;
pub use error::{AppError, AppErrorMessage};
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
    let git_router = Router::new()
        .route("/{owner}/{repo}/info/refs", get(git_info_refs))
        .route("/{owner}/{repo}/git-upload-pack", post(git_upload_pack))
        .route("/{owner}/{repo}/git-receive-pack", post(git_receive_pack));

    let org_router = Router::new().route("/organization/{org_name}", post(create_organization));

    let repo_router = Router::new()
        .route("/repository/{owner}/{repo}", post(create_repository))
        .route("/repository/{owner}/{repo}/tree", get(get_repository_tree))
        .route("/repository/{owner}/{repo}/file", get(get_repository_file))
        .route(
            "/repository/{owner}/{repo}/commits",
            get(get_repository_commits),
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
        .layer(middleware)
        .with_state(app_state)
}
