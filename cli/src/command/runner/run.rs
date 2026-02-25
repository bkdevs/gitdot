use gitdot_client::client::GitdotClient;

use crate::{
    config::RunnerConfig,
    executor::{Executor, ExecutorType, docker::DockerExecutor, local::LocalExecutor},
};

pub async fn run(config: RunnerConfig) -> anyhow::Result<()> {
    let token = match config.runner_token {
        Some(t) => t,
        None => {
            eprintln!("Error: runner is not installed. Please run `gitdot-runner install` first.");
            return Ok(());
        }
    };

    let client = GitdotClient::new("gitdot-runner")
        .with_token(token)
        .with_server_url(config.gitdot_server_url);

    loop {
        let task = match client.poll_task(()).await {
            Ok(Some(task)) => task,
            Ok(None) => continue,
            Err(e) => {
                eprintln!("Error polling task: {:#?}", e);
                continue;
            }
        };

        if let Err(e) = client.update_task(task.id, "running").await {
            eprintln!("Failed to mark task {} as running: {}", task.id, e);
            continue;
        }

        let result = match config.executor {
            ExecutorType::Local => {
                let executor = LocalExecutor {};
                executor.execute(&task).await
            }
            ExecutorType::Docker => {
                let executor = DockerExecutor;
                executor.execute(&task).await
            }
        };

        let final_status = match result {
            Ok(()) => "success",
            Err(ref e) => {
                eprintln!("Task {} failed: {}", task.id, e);
                "failure"
            }
        };

        if let Err(e) = client.update_task(task.id, final_status).await {
            eprintln!("Failed to mark task {} as {}: {}", task.id, final_status, e);
        }
    }
}
