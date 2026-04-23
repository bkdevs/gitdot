mod bootstrap;
mod error;
mod response;
mod settings;
mod state;

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{Router, routing::get};
use http::StatusCode;
use sqlx::PgPool;
use tokio::net;
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use gitdot_core::client::{GoogleSecretClient, SecretClient};

use crate::handler::create_auth_router;

pub use error::AppError;
pub use response::AppResponse;
pub use settings::Settings;
pub use state::AppState;

pub struct GitdotAuthServer {
    router: Router,
    listener: net::TcpListener,
}

impl GitdotAuthServer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Arc::new(Settings::new()?);
        let secret_client = GoogleSecretClient::new(settings.gcp_project_id.clone()).await?;

        let database_url = match &settings.database_url {
            Some(url) => url.clone(),
            None => secret_client.get_database_url().await?,
        };
        let pool = PgPool::connect(&database_url).await?;

        let state = AppState::new(pool, settings.clone(), secret_client).await?;
        let router = create_router(state);
        let listener = net::TcpListener::bind(&settings.get_server_address()).await?;

        Ok(Self { router, listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!(
            "Starting auth server on {}",
            self.listener.local_addr().unwrap()
        );
        axum::serve(
            self.listener,
            self.router
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .context("Failed to start auth server")?;
        Ok(())
    }
}

fn create_router(state: AppState) -> Router {
    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .expect("Failed to build governor config");

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list([
                    "https://gitdot.io".parse().unwrap(),
                    "http://localhost:3000".parse().unwrap(),
                ]))
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE, http::header::COOKIE])
                .allow_credentials(true),
        )
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(GovernorLayer {
            config: governor_config.into(),
        })
        .layer(PropagateRequestIdLayer::x_request_id());

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(create_auth_router())
        .layer(middleware)
        .with_state(state)
}
