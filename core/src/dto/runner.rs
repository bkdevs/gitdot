mod create_runner;
mod delete_runner;
mod register_runner;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{Runner, RunnerOwnerType};

pub use create_runner::CreateRunnerRequest;
pub use delete_runner::DeleteRunnerRequest;
pub use register_runner::RegisterRunnerRequest;

#[derive(Debug, Clone)]
pub struct RunnerResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub owner_type: RunnerOwnerType,
    pub created_at: DateTime<Utc>,
}

impl From<Runner> for RunnerResponse {
    fn from(runner: Runner) -> Self {
        Self {
            id: runner.id,
            name: runner.name,
            owner_id: runner.owner_id,
            owner_type: runner.owner_type,
            created_at: runner.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateRunnerResponse {
    pub runner: RunnerResponse,
    pub token: String,
}
