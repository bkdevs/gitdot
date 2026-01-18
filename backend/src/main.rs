mod app;
mod dto;
mod handlers;
mod utils;

use app::GitdotServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = GitdotServer::new().await?;
    server.start().await
}
