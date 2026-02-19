use anyhow::Result;
use gitdot_api::resource::TaskResource;

use crate::executor::{Executor, ExecutorType};

pub struct LocalExecutor;

impl Executor for LocalExecutor {
    const TYPE: ExecutorType = ExecutorType::Local;

    async fn execute(&self, task: &TaskResource) -> Result<()> {
        println!("LocalExecutor: executing task {}", task.id);
        Ok(())
    }
}
