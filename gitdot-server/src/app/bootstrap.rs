use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::Layer as _;

use gitdot_axum::bootstrap;

pub fn bootstrap() -> anyhow::Result<()> {
    bootstrap::load_env();
    bootstrap::install_crypto_provider();

    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("gitdot");
    opentelemetry::global::set_tracer_provider(provider);

    bootstrap::init_tracing_with(
        "info,tower_http=debug,axum::rejection=trace",
        vec![tracing_opentelemetry::layer().with_tracer(tracer).boxed()],
    );

    Ok(())
}
