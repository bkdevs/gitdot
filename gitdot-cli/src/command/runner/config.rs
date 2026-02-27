use crate::config::runner::RUNNER_CONFIG_PATH;

pub async fn config() -> anyhow::Result<()> {
    println!("{}", RUNNER_CONFIG_PATH);
    Ok(())
}
