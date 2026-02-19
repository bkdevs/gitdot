use gitdot_client::client::GitdotClient;

use crate::config::Config;
use crate::executor::docker::DockerExecutor;
use crate::executor::local::LocalExecutor;
use crate::executor::{Executor, ExecutorType};

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
            Ok(task) => match config.executor {
                ExecutorType::Local => {
                    let executor = LocalExecutor {
                        run_as_user: config.run_as_user.clone(),
                    };
                    if let Err(e) = executor.execute(&task).await {
                        eprintln!("Task {} failed: {}", task.id, e);
                    }
                }
                ExecutorType::Docker => {
                    let executor = DockerExecutor;
                    if let Err(e) = executor.execute(&task).await {
                        eprintln!("Task {} failed: {}", task.id, e);
                    }
                }
            },
            Err(e) => eprintln!("Error polling task: {}", e),
        }
    }
}
