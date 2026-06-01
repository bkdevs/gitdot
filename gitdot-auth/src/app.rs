mod bootstrap;
mod error;
mod response;
mod settings;
mod state;

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{Router, middleware::from_fn, routing::get};
use http::StatusCode;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio::net;
use tower::ServiceBuilder;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use gitdot_axum::middleware::log_request;

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
        let pool = PgPool::connect(settings.database_url.expose_secret()).await?;
        let state = AppState::new(pool, settings.clone()).await?;
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
        .per_second(20)
        .burst_size(100)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .expect("Failed to build governor config");

    let web_origin = state
        .settings
        .gitdot_web_url
        .parse()
        .expect("GITDOT_WEB_URL must be a valid origin");

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(log_request))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list([web_origin]))
                .allow_methods([http::Method::GET, http::Method::POST, http::Method::DELETE])
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
        .merge(create_auth_router(state.clone()))
        .layer(middleware)
        .with_state(state)
}
