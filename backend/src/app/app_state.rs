use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::client::{Git2Client, GitHttpClientImpl};
use gitdot_core::repository::{
    OAuthRepositoryImpl, OrganizationRepositoryImpl, QuestionRepositoryImpl,
    RepositoryRepositoryImpl, UserRepositoryImpl,
};
use gitdot_core::service::{
    AuthorizationService, AuthorizationServiceImpl, GitHttpService, GitHttpServiceImpl,
    OAuthService, OAuthServiceImpl, OrganizationService, OrganizationServiceImpl, QuestionService,
    QuestionServiceImpl, RepositoryService, RepositoryServiceImpl, UserService, UserServiceImpl,
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub auth_service: Arc<dyn AuthorizationService>,
    pub user_service: Arc<dyn UserService>,
    pub org_service: Arc<dyn OrganizationService>,
    pub repo_service: Arc<dyn RepositoryService>,
    pub question_service: Arc<dyn QuestionService>,
    pub git_http_service: Arc<dyn GitHttpService>,
    pub oauth_service: Arc<dyn OAuthService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let git_client = Git2Client::new(settings.git_project_root.clone());
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());

        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let question_repo = QuestionRepositoryImpl::new(pool.clone());
        let oauth_repo = OAuthRepositoryImpl::new(pool.clone());

        let auth_service = Arc::new(AuthorizationServiceImpl::new(
            org_repo.clone(),
            repo_repo.clone(),
            question_repo.clone(),
            user_repo.clone(),
        ));
        let user_service = Arc::new(UserServiceImpl::new(user_repo.clone(), repo_repo.clone()));
        let org_service = Arc::new(OrganizationServiceImpl::new(
            org_repo.clone(),
            user_repo.clone(),
            repo_repo.clone(),
        ));
        let repo_service = Arc::new(RepositoryServiceImpl::new(
            git_client.clone(),
            org_repo.clone(),
            repo_repo.clone(),
        ));
        let question_service = Arc::new(QuestionServiceImpl::new(
            question_repo.clone(),
            repo_repo.clone(),
        ));
        let git_http_service = Arc::new(GitHttpServiceImpl::new(git_http_client.clone()));
        let oauth_service = Arc::new(OAuthServiceImpl::new(oauth_repo.clone(), user_repo.clone()));

        Self {
            settings,
            auth_service,
            user_service,
            org_service,
            repo_service,
            question_service,
            git_http_service,
            oauth_service,
        }
    }
}
