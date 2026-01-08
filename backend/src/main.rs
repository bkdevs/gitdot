use gitdot::app::server::GitdotServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = GitdotServer::new().await?;
    server.start().await
}
