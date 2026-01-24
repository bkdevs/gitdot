use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::client::{Git2Client, GitHttpClientImpl};
use gitdot_core::repository::{
    OrganizationRepositoryImpl, RepositoryRepositoryImpl, UserRepositoryImpl,
};
use gitdot_core::service::{
    AuthorizationService, AuthorizationServiceImpl, GitHttpService, GitHttpServiceImpl,
    OrganizationService, OrganizationServiceImpl, RepositoryService, RepositoryServiceImpl,
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub auth_service: Arc<dyn AuthorizationService>,
    pub org_service: Arc<dyn OrganizationService>,
    pub repo_service: Arc<dyn RepositoryService>,
    pub git_http_service: Arc<dyn GitHttpService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let git_client = Git2Client::new(settings.git_project_root.clone());
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());

        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());

        let auth_service = Arc::new(AuthorizationServiceImpl::new(
            org_repo.clone(),
            repo_repo.clone(),
        ));
        let org_service = Arc::new(OrganizationServiceImpl::new(
            org_repo.clone(),
            user_repo.clone(),
        ));
        let repo_service = Arc::new(RepositoryServiceImpl::new(
            git_client.clone(),
            org_repo.clone(),
            repo_repo.clone(),
        ));
        let git_http_service = Arc::new(GitHttpServiceImpl::new(git_http_client));

        Self {
            settings,
            auth_service,
            org_service,
            repo_service,
            git_http_service,
        }
    }
}
