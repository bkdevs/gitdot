use crate::config::Config;

pub async fn login(mut config: Config) -> anyhow::Result<()> {
    config.user_name = "mikkel".to_string();
    config.save().await?;
    Ok(())
}
