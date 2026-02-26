use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn bootstrap() -> anyhow::Result<()> {
    load_env()?;
    load_rustls()?;
    init_tracing()?;
    Ok(())
}

fn load_env() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    Ok(())
}

fn load_rustls() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");
    Ok(())
}

fn init_tracing() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "info,gitdot=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
        )
        .init();
    Ok(())
}
