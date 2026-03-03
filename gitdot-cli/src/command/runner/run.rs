use std::sync::Arc;

use anyhow::Context;

use crate::{
    client::GitdotClient,
    config::RunnerConfig,
    executor::{Executor, local::LocalExecutor},
};

pub async fn run(config: RunnerConfig) -> anyhow::Result<()> {
    if config.runner_token.is_none() {
        eprintln!("Error: runner is not installed. Please run `gitdot-runner install` first.");
        return Ok(());
    }

    let config = Arc::new(config);
    let runner_client = Arc::new(GitdotClient::from_runner_config(&config));
    let semaphore = Arc::new(tokio::sync::Semaphore::new(config.num_executors as usize));

    loop {
        let permit = Arc::clone(&semaphore)
            .acquire_owned()
            .await
            .context("semaphore closed")?;

        let task = match runner_client.poll_task(()).await {
            Ok(Some(task)) => task,
            Ok(None) => {
                drop(permit);
                continue;
            }
            Err(e) => {
                eprintln!("Error polling task: {:#?}", e);
                drop(permit);
                continue;
            }
        };

        let task_client =
            Arc::new(GitdotClient::from_runner_config(&config).with_jwt(task.token.clone()));
        let config = Arc::clone(&config);

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = task_client.update_task(task.id, "running").await {
                eprintln!("Failed to mark task {} as running: {}", task.id, e);
                return;
            }

            let executor = match LocalExecutor::initialize(&config, &task).await {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Failed to initialize executor: {}", e);
                    return;
                }
            };

            let task_id = executor.task.id;
            let result = executor.execute().await;

            let final_status = match result {
                Ok(()) => "success",
                Err(ref e) => {
                    eprintln!("Task {} failed: {}", task_id, e);
                    "failure"
                }
            };

            if let Err(e) = task_client.update_task(task_id, final_status).await {
                eprintln!("Failed to mark task {} as {}: {}", task_id, final_status, e);
            }

            if let Err(e) = executor.cleanup().await {
                eprintln!("Failed to clean up task {}: {}", task_id, e);
            }
        });
    }
}
