use ci_server::CiServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = CiServer::new().await?;
    server.start().await
}
