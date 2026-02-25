use anyhow::Context;
use gitdot_client::client::GitdotClient;
use s2_sdk::{
    S2,
    types::{AccountEndpoint, BasinEndpoint, S2Config, S2Endpoints},
};

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

    let account_endpoint: AccountEndpoint = config
        .s2_server_url
        .parse()
        .context("invalid s2_server_url (account endpoint)")?;
    let basin_endpoint: BasinEndpoint = config
        .s2_server_url
        .parse()
        .context("invalid s2_server_url (basin endpoint)")?;
    let s2_endpoints = S2Endpoints::new(account_endpoint, basin_endpoint)
        .context("failed to build S2 endpoints")?;
    let s2 = S2::new(S2Config::new().with_endpoints(s2_endpoints))
        .context("failed to init S2 client")?;

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
                let executor = LocalExecutor::new(s2.clone());
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
