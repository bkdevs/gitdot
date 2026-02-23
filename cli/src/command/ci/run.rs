use gitdot_client::client::GitdotClient;

use crate::{
    config::Config,
    executor::{Executor, ExecutorType, docker::DockerExecutor, local::LocalExecutor},
};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let token = match config.ci.runner_token {
        Some(t) => t,
        None => {
            eprintln!("Error: runner is not installed. Please run `gitdot-runner install` first.");
            return Ok(());
        }
    };

    let client = GitdotClient::new("gitdot-runner".to_string()).with_token(token);

    loop {
        match client.poll_task(()).await {
            Ok(task) => match config.ci.executor {
                ExecutorType::Local => {
                    let executor = LocalExecutor {};
                    println!("{:?}", task);
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
