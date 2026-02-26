use clap::Parser;

use gitdot_cli::{Args, bootstrap, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::load_rustls()?;
    let args = Args::parse();
    run(&args).await
}
