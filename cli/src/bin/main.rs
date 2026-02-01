use clap::Parser;

use gitdot_cli::{Args, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(&args).await
}
