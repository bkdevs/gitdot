use std::sync::Arc;

use axum::extract::FromRef;
use gitdot_core::{
    repository::{OrganizationRepositoryImpl, RunnerRepositoryImpl},
    service::{RunnerService, RunnerServiceImpl},
};
use sqlx::PgPool;

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub runner_service: Arc<dyn RunnerService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let runner_repo = RunnerRepositoryImpl::new(pool.clone());

        let repo_service = Arc::new(RunnerServiceImpl::new(runner_repo, org_repo));

        Self {
            settings,
            runner_service: repo_service,
        }
    }
}
