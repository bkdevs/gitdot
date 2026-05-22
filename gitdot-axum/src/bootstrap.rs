use std::io::IsTerminal as _;

use tracing_subscriber::{
    Layer, Registry, fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

/// Load environment variables from a `.env` file if one is present.
///
/// A missing `.env` is not an error — in production the process is configured
/// via injected environment variables instead.
pub fn load_env() {
    dotenvy::dotenv().ok();
}

/// Install the process-wide rustls crypto provider (aws-lc-rs).
///
/// Must be called once at startup, before any TLS connections are made.
pub fn install_crypto_provider() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");
}

/// Initialize `tracing` with a terminal-aware fmt layer.
///
/// `default_filter` is used as the `EnvFilter` directive when `RUST_LOG` is
/// unset. The fmt layer is pretty/colored on a TTY (local dev) and JSON
/// otherwise (e.g. Cloud Run, parsed by GCP Logs Explorer).
pub fn init_tracing(default_filter: &str) {
    init_tracing_with(default_filter, Vec::new());
}

/// Like [`init_tracing`], but appends caller-supplied subscriber layers — for
/// example an OpenTelemetry bridge layer.
pub fn init_tracing_with(
    default_filter: &str,
    extra_layers: Vec<Box<dyn Layer<Registry> + Send + Sync>>,
) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| default_filter.into());

    let fmt_layer: Box<dyn Layer<Registry> + Send + Sync> = if std::io::stdout().is_terminal() {
        // local dev: pretty, colored, multi-line
        tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_span_events(FmtSpan::CLOSE)
            .boxed()
    } else {
        // prod (e.g. Cloud Run): JSON, no ANSI, parsed by GCP Logs Explorer
        tracing_subscriber::fmt::layer()
            .json()
            .with_target(true)
            .with_current_span(true)
            .with_span_list(false)
            .with_span_events(FmtSpan::CLOSE)
            .boxed()
    };

    let mut layers = vec![fmt_layer];
    layers.extend(extra_layers);

    tracing_subscriber::registry()
        .with(layers)
        .with(env_filter)
        .init();
}
