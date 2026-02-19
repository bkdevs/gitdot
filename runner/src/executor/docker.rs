use anyhow::Result;
use gitdot_api::resource::TaskResource;

use crate::executor::{Executor, ExecutorType};

pub struct DockerExecutor;

impl Executor for DockerExecutor {
    const TYPE: ExecutorType = ExecutorType::Docker;

    async fn execute(&self, task: &TaskResource) -> Result<()> {
        println!("DockerExecutor: executing task {}", task.id);
        Ok(())
    }
}
