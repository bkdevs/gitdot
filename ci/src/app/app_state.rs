use std::sync::Arc;

use axum::extract::FromRef;
use gitdot_core::{
    repository::{DagRepositoryImpl, OrganizationRepositoryImpl, RunnerRepositoryImpl},
    service::{DagService, DagServiceImpl, RunnerService, RunnerServiceImpl},
};
use sqlx::PgPool;

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub runner_service: Arc<dyn RunnerService>,
    pub dag_service: Arc<dyn DagService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let runner_repo = RunnerRepositoryImpl::new(pool.clone());
        let dag_repo = DagRepositoryImpl::new(pool.clone());

        let runner_service = Arc::new(RunnerServiceImpl::new(runner_repo, org_repo));
        let dag_service = Arc::new(DagServiceImpl::new(dag_repo));

        Self {
            settings,
            runner_service,
            dag_service,
        }
    }
}
