use crate::{config::Config, executor::ExecutorType, os::install_service};
use gitdot_client::client::GitdotClient;
use std::io::{self, Write};

pub async fn install(mut config: Config) -> anyhow::Result<()> {
    if config.ci.runner_token.is_none() {
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

        config.ci.runner_token = Some(token);
        config.save().await?;
    }

    print!("Select executor [local/docker] [local]: ");
    io::stdout().flush()?;
    let mut executor_input = String::new();
    io::stdin().read_line(&mut executor_input)?;
    let executor = match executor_input.trim().to_lowercase().as_str() {
        "docker" => ExecutorType::Docker,
        "local" | "" => ExecutorType::Local,
        other => anyhow::bail!("Unknown executor '{}'. Must be 'local' or 'docker'.", other),
    };
    config.ci.executor = executor;
    config.save().await?;

    install_service()?;

    Ok(())
}
