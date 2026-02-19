use std::io::{self, Write};

use gitdot_client::client::GitdotClient;

use crate::config::Config;

pub async fn install(mut config: Config) -> anyhow::Result<()> {
    print!("Have you created a runner in the gitdot UI? (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if matches!(input.trim().to_lowercase().as_str(), "n" | "no") {
        println!("Please create a runner at gitdot.io/settings/runners to continue.");
        return Ok(());
    }

    print!("Please paste your runner token: ");
    io::stdout().flush()?;
    let mut token_input = String::new();
    io::stdin().read_line(&mut token_input)?;

    let token = token_input.trim().to_string();

    println!("Verifying your runner token...");

    let client = GitdotClient::new("gitdot-runner".to_string()).with_token(token.clone());
    client.verify_runner().await?;

    config.runner_token = Some(token);
    config.save().await?;

    println!("Runner installed successfully.");

    Ok(())
}
