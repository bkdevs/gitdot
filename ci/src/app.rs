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
use axum::{Router, routing::get};
use tokio::net;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::handler::{create_dag_router, create_runner_router, create_task_router};

pub use app_state::AppState;
pub use auth::AuthenticatedUser;
pub use error::AppError;
pub use response::AppResponse;
pub use settings::Settings;

pub struct CiServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl CiServer {
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
    let runner_router = create_runner_router();
    let dag_router = create_dag_router();
    let task_router = create_task_router();

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(PropagateRequestIdLayer::x_request_id());

    let api_router = Router::new()
        .merge(runner_router)
        .merge(dag_router)
        .merge(task_router)
        .layer(middleware)
        .with_state(app_state);

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(api_router)
}
