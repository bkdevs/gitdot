use std::sync::Arc;

use anyhow::Context;

use crate::{client::GitdotClient, config::RunnerConfig, executor::state::ExecutorState};

pub async fn run(config: RunnerConfig) -> anyhow::Result<()> {
    if config.runner_token.is_none() {
        eprintln!("Error: runner is not installed. Please run `gitdot-runner install` first.");
        return Ok(());
    }

    let config = Arc::new(config);
    let client = Arc::new(GitdotClient::from_runner_config(&config));
    let semaphore = Arc::new(tokio::sync::Semaphore::new(config.num_executors as usize));

    loop {
        let permit = Arc::clone(&semaphore)
            .acquire_owned()
            .await
            .context("semaphore closed")?;

        let task = match client.poll_task(()).await {
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

        let client = Arc::clone(&client);
        let config = Arc::clone(&config);

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = client.update_task(task.id, "running").await {
                eprintln!("Failed to mark task {} as running: {}", task.id, e);
                return;
            }

            let state = match ExecutorState::initialize(&config, task).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to initialize executor state: {}", e);
                    return;
                }
            };

            let task_id = state.task.id;
            let result = config.executor.execute(state).await;

            let final_status = match result {
                Ok(()) => "success",
                Err(ref e) => {
                    eprintln!("Task {} failed: {}", task_id, e);
                    "failure"
                }
            };

            if let Err(e) = client.update_task(task_id, final_status).await {
                eprintln!("Failed to mark task {} as {}: {}", task_id, final_status, e);
            }
        });
    }
}
