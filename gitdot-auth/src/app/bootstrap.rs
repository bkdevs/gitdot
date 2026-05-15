use std::io::IsTerminal as _;

use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn bootstrap() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=debug,axum::rejection=trace".into());
    let fmt_layer = if std::io::stdout().is_terminal() {
        // local dev: pretty, colored, multi-line
        tracing_subscriber::fmt::layer().with_target(true).boxed()
    } else {
        // prod (e.g. Cloud Run): JSON, no ANSI, parsed by GCP Logs Explorer
        tracing_subscriber::fmt::layer()
            .json()
            .with_target(true)
            .with_current_span(true)
            .with_span_list(false)
            .boxed()
    };
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    Ok(())
}
