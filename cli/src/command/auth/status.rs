use crate::config::{AuthStatus, Config};

pub async fn get_status(config: Config) -> anyhow::Result<()> {
    match config.get_auth_status() {
        AuthStatus::LoggedIn { user_name } => {
            println!("Logged in as {}", user_name);
        }
        AuthStatus::LoggedOut => {
            println!("Not logged in");
        }
    }
    Ok(())
}
