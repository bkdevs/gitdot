use clap::Parser;

use gitdot_cli::{bootstrap, Args, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::load_rustls()?;
    let args = Args::parse();
    run(&args).await
}
