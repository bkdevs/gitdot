use crate::config::UserConfig;

pub async fn get_status(config: UserConfig) -> anyhow::Result<()> {
    if config.profile.user_name.is_empty() {
        println!("Not logged in");
    } else {
        println!("Logged in as {}", config.profile.user_name);
    }
    Ok(())
}
