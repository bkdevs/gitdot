use gitdot_axum::bootstrap;

pub fn bootstrap() -> anyhow::Result<()> {
    bootstrap::load_env();
    bootstrap::install_crypto_provider();
    bootstrap::init_tracing("info,tower_http=debug,axum::rejection=trace");
    Ok(())
}
