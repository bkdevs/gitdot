use std::sync::Arc;

use anyhow::Context;
use tokio::net;

use super::app_state::AppState;
use super::bootstrap::bootstrap;
use super::router::create_router;
use super::settings::Settings;

pub struct GitdotServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl GitdotServer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap()?;

        let settings = Arc::new(Settings::new()?);
        let address = settings.get_server_address();

        let state = AppState::new(settings);
        let router = create_router(state);
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

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
