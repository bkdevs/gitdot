use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::client::{Git2Client, GitHttpClientImpl};
use gitdot_core::repository::{
    CommitRepositoryImpl, DagRepositoryImpl, OrganizationRepositoryImpl, QuestionRepositoryImpl,
    RepositoryRepositoryImpl, RunnerRepositoryImpl, TaskRepositoryImpl, TokenRepositoryImpl,
    UserRepositoryImpl,
};
use gitdot_core::service::{
    AuthorizationService, AuthorizationServiceImpl, CommitService, CommitServiceImpl, DagService,
    DagServiceImpl, GitHttpService, GitHttpServiceImpl, OrganizationService,
    OrganizationServiceImpl, QuestionService, QuestionServiceImpl, RepositoryService,
    RepositoryServiceImpl, RunnerService, RunnerServiceImpl, TaskService, TaskServiceImpl,
    TokenService, TokenServiceImpl, UserService, UserServiceImpl,
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub auth_service: Arc<dyn AuthorizationService>,
    pub token_service: Arc<dyn TokenService>,

    pub git_http_service: Arc<dyn GitHttpService>,
    pub user_service: Arc<dyn UserService>,
    pub org_service: Arc<dyn OrganizationService>,
    pub repo_service: Arc<dyn RepositoryService>,
    pub question_service: Arc<dyn QuestionService>,
    pub commit_service: Arc<dyn CommitService>,

    pub runner_service: Arc<dyn RunnerService>,
    pub dag_service: Arc<dyn DagService>,
    pub task_service: Arc<dyn TaskService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let git_client = Git2Client::new(settings.git_project_root.clone());
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());

        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let question_repo = QuestionRepositoryImpl::new(pool.clone());
        let commit_repo = CommitRepositoryImpl::new(pool.clone());
        let runner_repo = RunnerRepositoryImpl::new(pool.clone());
        let dag_repo = DagRepositoryImpl::new(pool.clone());
        let task_repo = TaskRepositoryImpl::new(pool.clone());

        let auth_service = Arc::new(AuthorizationServiceImpl::new(
            org_repo.clone(),
            repo_repo.clone(),
            question_repo.clone(),
            user_repo.clone(),
        ));
        let token_service = Arc::new(TokenServiceImpl::new(token_repo.clone(), user_repo.clone()));
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
            user_repo.clone(),
        ));
        let question_service = Arc::new(QuestionServiceImpl::new(
            question_repo.clone(),
            repo_repo.clone(),
        ));
        let commit_service = Arc::new(CommitServiceImpl::new(commit_repo.clone()));
        let git_http_service = Arc::new(GitHttpServiceImpl::new(git_http_client.clone()));
        let runner_service = Arc::new(RunnerServiceImpl::new(
            runner_repo.clone(),
            org_repo.clone(),
            token_repo.clone(),
        ));
        let dag_service = Arc::new(DagServiceImpl::new(dag_repo.clone()));
        let task_service = Arc::new(TaskServiceImpl::new(task_repo.clone()));

        Self {
            settings,
            auth_service,
            token_service,
            user_service,
            org_service,
            repo_service,
            question_service,
            commit_service,
            git_http_service,
            runner_service,
            dag_service,
            task_service,
        }
    }
}
