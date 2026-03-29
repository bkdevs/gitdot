use gitdot_auth::GitdotAuthServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = GitdotAuthServer::new().await?;
    server.start().await
}
