use crate::{client::GitdotClient, config::RunnerConfig};

pub async fn verify(config: RunnerConfig) -> anyhow::Result<()> {
    let client = GitdotClient::from_runner_config(&config);

    match client.verify_runner().await {
        Ok(()) => println!("Runner config verified."),
        Err(_) => println!("Runner config invalid."),
    }

    Ok(())
}
