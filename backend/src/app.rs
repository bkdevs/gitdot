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
use axum::{Router, routing::get};
use tokio::net;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

#[cfg(feature = "main")]
use crate::handler::{
    create_git_http_router, create_oauth_router, create_organization_router,
    create_question_router, create_repository_router, create_user_router,
};

#[cfg(feature = "ci")]
use crate::handler::{create_dag_router, create_runner_router, create_task_router};

pub use app_state::AppState;
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

fn create_router(app_state: AppState) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // TODO: update CORS policy
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(PropagateRequestIdLayer::x_request_id());

    let mut api_router = Router::new();
    let mut git_router = Router::new();

    #[cfg(feature = "main")]
    {
        api_router = api_router
            .merge(create_user_router())
            .merge(create_organization_router())
            .merge(create_repository_router())
            .merge(create_question_router())
            .merge(create_oauth_router());
        git_router = git_router.merge(create_git_http_router());
    }

    #[cfg(feature = "ci")]
    {
        api_router = api_router.nest(
            "/ci",
            Router::new()
                .merge(create_runner_router())
                .merge(create_dag_router())
                .merge(create_task_router()),
        );
    }

    let api_router = api_router.layer(middleware).with_state(app_state.clone());
    let git_router = git_router.with_state(app_state);

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(api_router)
        .merge(git_router)
}
