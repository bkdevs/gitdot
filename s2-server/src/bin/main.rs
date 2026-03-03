#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn install_rustls_crypto_provider() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install aws-lc-rs as default rustls crypto provider");
}

#[derive(Parser, Debug)]
#[command(author, version, about = "S2 Lite")]
struct Args {
    #[command(flatten)]
    lite: s2_server::server::LiteArgs,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    install_rustls_crypto_provider();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    std::env::var("GITDOT_PUBLIC_KEY").expect("GITDOT_PUBLIC_KEY must be set");

    let args = Args::parse();
    s2_server::server::run(args.lite).await
}
