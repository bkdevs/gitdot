use clap::Parser;

use gitdot_runner::{Args, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(&args).await
}
