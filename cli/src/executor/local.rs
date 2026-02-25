use std::process::Stdio;

use anyhow::{Context, Result};
use gitdot_api::resource::TaskResource;
use tokio::process::Command;

use crate::executor::{Executor, ExecutorType};

pub struct LocalExecutor {}

impl Executor for LocalExecutor {
    const TYPE: ExecutorType = ExecutorType::Local;

    async fn execute(&self, task: &TaskResource) -> Result<()> {
        let output = Command::new("sh")
            .args(["-c", &task.command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to spawn process")?;

        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            anyhow::bail!("Task {} exited with status {}", task.id, output.status);
        }

        Ok(())
    }
}
