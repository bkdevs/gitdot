use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn bootstrap() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,gitdot_auth=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    Ok(())
}
