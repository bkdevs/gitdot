use crate::{
    config::Config,
    executor::ExecutorType,
    service::{Service, ServiceManager},
};
use gitdot_client::client::GitdotClient;
use std::io::{self, Write};

mod create_user;
mod user_exists;

use create_user::create_user;
use user_exists::user_exists;

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

    #[cfg(target_os = "macos")]
    let default_user = std::env::var("USER").unwrap_or_else(|_| "gitdot-runner".to_string());
    #[cfg(not(target_os = "macos"))]
    let default_user = "gitdot-runner".to_string();

    print!("Run tasks as user [{default_user}]: ");
    io::stdout().flush()?;

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let run_as_user = {
        let trimmed = user_input.trim();
        if trimmed.is_empty() {
            default_user
        } else {
            trimmed.to_string()
        }
    };

    if run_as_user == "gitdot-runner" {
        create_user("gitdot-runner")?;
    } else if !user_exists(&run_as_user) {
        anyhow::bail!(
            "User '{}' does not exist. Please create the user before running install.",
            run_as_user
        );
    }

    config.run_as_user = run_as_user;
    config.save().await?;

    print!("Select executor [local/docker] [local]: ");
    io::stdout().flush()?;
    let mut executor_input = String::new();
    io::stdin().read_line(&mut executor_input)?;
    let executor = match executor_input.trim().to_lowercase().as_str() {
        "docker" => ExecutorType::Docker,
        "local" | "" => ExecutorType::Local,
        other => anyhow::bail!("Unknown executor '{}'. Must be 'local' or 'docker'.", other),
    };
    config.executor = executor;
    config.save().await?;

    println!("Installing OS service...");
    let manager = ServiceManager::new(config.run_as_user.clone())?;
    manager.install()?;
    println!("Runner installed.");
    Ok(())
}
