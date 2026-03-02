use std::{path::PathBuf, process::Stdio};

use anyhow::{Context, Result};
use gitdot_api::resource::PollTaskResource;
use s2_sdk::{
    S2,
    producer::ProducerConfig,
    types::{AppendRecord, Header},
};
use tokio::{io::AsyncReadExt, process::Command};

use crate::{config::RunnerConfig, executor::Executor};

pub struct LocalExecutor {
    pub working_directory: PathBuf,
    pub task: PollTaskResource,
    pub s2: S2,
}

impl Executor for LocalExecutor {
    async fn initialize(config: &RunnerConfig, task: &PollTaskResource) -> Result<Self> {
        let working_directory = PathBuf::from(format!("/tmp/gitdot/tasks/{}", task.id));
        tokio::fs::create_dir_all(&working_directory).await?;

        let clone_url = format!(
            "{}/{}/{}",
            config.gitdot_server_url, task.owner_name, task.repository_name
        );
        let clone_dir = working_directory.clone();
        tokio::task::spawn_blocking(move || {
            git2::Repository::clone(&clone_url, &clone_dir).context("Failed to clone repository")
        })
        .await??;

        Ok(Self {
            working_directory,
            task: task.clone(),
            s2: S2::from_url(&config.s2_server_url)?,
        })
    }

    async fn execute(&self) -> Result<()> {
        let (basin_name, stream_name) = crate::util::s2::parse_s2_uri(&self.task.s2_uri)?;

        let stream = self.s2.basin(basin_name).stream(stream_name);
        let producer = stream.producer(ProducerConfig::default());

        let mut child = Command::new("sh")
            .args(["-c", &self.task.command])
            .current_dir(&self.working_directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn process")?;

        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        let mut stdout_buf = [0u8; 8192];
        let mut stderr_buf = [0u8; 8192];
        let mut stdout_done = false;
        let mut stderr_done = false;

        while !stdout_done || !stderr_done {
            tokio::select! {
                result = stdout.read(&mut stdout_buf), if !stdout_done => {
                    let n = result?;
                    if n == 0 {
                        stdout_done = true;
                    } else {
                        let record = AppendRecord::new(stdout_buf[..n].to_vec())?
                            .with_headers([Header::new("stream", "stdout")])?;
                        producer.submit(record).await?;
                    }
                },
                result = stderr.read(&mut stderr_buf), if !stderr_done => {
                    let n = result?;
                    if n == 0 {
                        stderr_done = true;
                    } else {
                        let record = AppendRecord::new(stderr_buf[..n].to_vec())?
                            .with_headers([Header::new("stream", "stderr")])?;
                        producer.submit(record).await?;
                    }
                },
            }
        }

        let status = child.wait().await?;

        let task_status = if status.success() {
            "success"
        } else {
            "failure"
        };

        let record =
            AppendRecord::new(vec![])?.with_headers([Header::new("task-finished", task_status)])?;
        producer.submit(record).await?;
        producer.close().await?;

        if !status.success() {
            anyhow::bail!("Task {} exited with status {}", self.task.id, status);
        }
        Ok(())
    }

    async fn cleanup(self) -> Result<()> {
        tokio::fs::remove_dir_all(&self.working_directory).await?;
        Ok(())
    }
}
