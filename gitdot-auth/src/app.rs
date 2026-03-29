mod bootstrap;
mod settings;
mod state;

use anyhow::Context;
use axum::{Router, routing::get};
use sqlx::PgPool;
use tokio::net;

use crate::handler::create_auth_router;

pub use settings::Settings;
pub use state::AppState;

pub struct GitdotAuthServer {
    router: Router,
    listener: net::TcpListener,
}

impl GitdotAuthServer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Settings::new()?;
        let pool = PgPool::connect(&settings.database_url).await?;

        let state = AppState::new(pool);
        let router = create_router(state);
        let listener = net::TcpListener::bind(&settings.get_server_address()).await?;

        Ok(Self { router, listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!(
            "Starting auth server on {}",
            self.listener.local_addr().unwrap()
        );
        axum::serve(self.listener, self.router)
            .await
            .context("Failed to start auth server")?;
        Ok(())
    }
}

fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(create_auth_router())
        .with_state(state)
}
