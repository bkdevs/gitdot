use std::process::Stdio;

use anyhow::{Context, Result};
use gitdot_api::resource::PollTaskResource;
use s2_sdk::{
    S2,
    producer::ProducerConfig,
    types::{AppendRecord, Header},
};
use tokio::{io::AsyncReadExt, process::Command};

use crate::executor::{Executor, ExecutorType};

pub struct LocalExecutor {
    s2: S2,
}

impl LocalExecutor {
    pub fn new(s2: S2) -> Self {
        Self { s2 }
    }
}

impl Executor for LocalExecutor {
    const TYPE: ExecutorType = ExecutorType::Local;

    async fn execute(&self, task: &PollTaskResource) -> Result<()> {
        let (basin_name, stream_name) = crate::util::s2::parse_s2_uri(&task.s2_uri)?;

        let stream = self.s2.basin(basin_name).stream(stream_name);
        let producer = stream.producer(ProducerConfig::default());

        let mut child = Command::new("sh")
            .args(["-c", &task.command])
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

        producer.close().await?;
        let status = child.wait().await?;

        if !status.success() {
            anyhow::bail!("Task {} exited with status {}", task.id, status);
        }
        Ok(())
    }
}
