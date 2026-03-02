use anyhow::Result;

use crate::executor::state::ExecutorState;

pub async fn execute(state: ExecutorState) -> Result<()> {
    println!("DockerExecutor: executing task {}", state.task.id);
    Ok(())
}
