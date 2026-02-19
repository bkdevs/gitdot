use gitdot_client::client::GitdotClient;

use crate::config::Config;

pub async fn run(config: Config) -> anyhow::Result<()> {
    let token = match config.runner_token {
        Some(t) => t,
        None => {
            eprintln!("Error: runner is not installed. Please run `gitdot-runner install` first.");
            return Ok(());
        }
    };

    let client = GitdotClient::new("gitdot-runner".to_string()).with_token(token);

    loop {
        match client.poll_task(()).await {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Error polling task: {}", e),
        }
    }
}
