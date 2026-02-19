mod create_runner;
mod create_runner_token;
mod delete_runner;
mod get_runner;
mod list_runners;
mod verify_runner;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{Runner, RunnerOwnerType};

pub use create_runner::CreateRunnerRequest;
pub use create_runner_token::{CreateRunnerTokenRequest, CreateRunnerTokenResponse};
pub use delete_runner::DeleteRunnerRequest;
pub use get_runner::GetRunnerRequest;
pub use list_runners::ListRunnersRequest;
pub use verify_runner::VerifyRunnerRequest;

#[derive(Debug, Clone)]
pub struct RunnerResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub owner_name: String,
    pub owner_type: RunnerOwnerType,
    pub last_verified: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Runner> for RunnerResponse {
    fn from(runner: Runner) -> Self {
        Self {
            id: runner.id,
            name: runner.name,
            owner_id: runner.owner_id,
            owner_name: runner.owner_name,
            owner_type: runner.owner_type,
            last_verified: runner.last_verified,
            created_at: runner.created_at,
        }
    }
}

pub type CreateRunnerResponse = RunnerResponse;
pub type GetRunnerResponse = RunnerResponse;
pub type ListRunnersResponse = Vec<RunnerResponse>;
