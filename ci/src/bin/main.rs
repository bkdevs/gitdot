use anyhow::Context;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Failed to start server")?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
