use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::client::{Git2Client, GitHttpBackendClientImpl};
use gitdot_core::repository::{
    OrganizationRepositoryImpl, RepositoryRepositoryImpl, UserRepositoryImpl,
};
use gitdot_core::service::{
    GitHttpBackendService, GitHttpBackendServiceImpl, OrganizationService, OrganizationServiceImpl,
    RepositoryService, RepositoryServiceImpl,
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub org_service: Arc<dyn OrganizationService>,
    pub repo_service: Arc<dyn RepositoryService>,
    pub git_http_service: Arc<dyn GitHttpBackendService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let git_client = Git2Client::new(settings.git_project_root.clone());
        let git_http_client = GitHttpBackendClientImpl::new(settings.git_project_root.clone());

        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());

        let org_service = Arc::new(OrganizationServiceImpl::new(
            org_repo.clone(),
            user_repo.clone(),
        ));
        let repo_service = Arc::new(RepositoryServiceImpl::new(
            git_client.clone(),
            org_repo.clone(),
            repo_repo.clone(),
        ));
        let git_http_service = Arc::new(GitHttpBackendServiceImpl::new(git_http_client));

        Self {
            settings,
            org_service,
            repo_service,
            git_http_service,
        }
    }
}
