use std::io::IsTerminal as _;

use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

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
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=debug,axum::rejection=trace".into());

    let fmt_layer = if std::io::stdout().is_terminal() {
        // local dev: pretty, colored, multi-line
        tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .boxed()
    } else {
        // prod (e.g. Cloud Run): JSON, no ANSI, parsed by GCP Logs Explorer
        tracing_subscriber::fmt::layer()
            .json()
            .with_target(true)
            .with_current_span(true)
            .with_span_list(false)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .boxed()
    };

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    #[cfg(feature = "otel")]
    let registry = {
        use opentelemetry::trace::TracerProvider as _;
        use opentelemetry_sdk::trace::SdkTracerProvider;

        let provider = SdkTracerProvider::builder()
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .build();
        let tracer = provider.tracer("gitdot");
        opentelemetry::global::set_tracer_provider(provider);
        registry.with(tracing_opentelemetry::layer().with_tracer(tracer))
    };

    registry.init();
    Ok(())
}
