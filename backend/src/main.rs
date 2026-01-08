use gitdot::app::GitdotServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = GitdotServer::new().await?;
    server.start().await
}
