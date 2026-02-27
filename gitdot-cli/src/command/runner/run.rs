use std::sync::Arc;

use crate::client::GitdotClient;
use anyhow::Context;
use s2_sdk::S2;

use crate::{
    config::RunnerConfig,
    executor::{Executor, ExecutorType, docker::DockerExecutor, local::LocalExecutor},
};

pub async fn run(config: RunnerConfig) -> anyhow::Result<()> {
    if config.runner_token.is_none() {
        eprintln!("Error: runner is not installed. Please run `gitdot-runner install` first.");
        return Ok(());
    }

    let client = Arc::new(GitdotClient::from_runner_config(&config));
    let s2 = S2::from_url(&config.s2_server_url).context("failed to init S2 client")?;
    let executor_type = config.executor;

    // TODO(temp): naive multi-task runner — revisit concurrency model
    loop {
        let task = match client.poll_task(()).await {
            Ok(Some(task)) => task,
            Ok(None) => continue,
            Err(e) => {
                eprintln!("Error polling task: {:#?}", e);
                continue;
            }
        };

        let client = Arc::clone(&client);
        let s2 = s2.clone();

        tokio::spawn(async move {
            if let Err(e) = client.update_task(task.id, "running").await {
                eprintln!("Failed to mark task {} as running: {}", task.id, e);
                return;
            }

            let result = match executor_type {
                ExecutorType::Local => {
                    let executor = LocalExecutor::new(s2);
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
        });
    }
}
