use clap::Parser;

use gitdot_cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    args.run().await
}
