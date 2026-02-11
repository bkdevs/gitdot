use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::repository::TokenRepositoryImpl;
use gitdot_core::service::{
    AuthorizationService, AuthorizationServiceImpl, TokenService, TokenServiceImpl,
};

cfg_use!("main", {
    use gitdot_core::client::{Git2Client, GitHttpClientImpl};
    use gitdot_core::repository::{
        CommitRepositoryImpl, OrganizationRepositoryImpl, QuestionRepositoryImpl,
        RepositoryRepositoryImpl, UserRepositoryImpl,
    };
    use gitdot_core::service::{
        CommitService, CommitServiceImpl, GitHttpService, GitHttpServiceImpl, OrganizationService,
        OrganizationServiceImpl, QuestionService, QuestionServiceImpl, RepositoryService,
        RepositoryServiceImpl, UserService, UserServiceImpl,
    };
});

cfg_use!("ci", {
    use gitdot_core::repository::{DagRepositoryImpl, RunnerRepositoryImpl, TaskRepositoryImpl};
    use gitdot_core::service::{
        DagService, DagServiceImpl, RunnerService, RunnerServiceImpl, TaskService, TaskServiceImpl,
    };
});

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub auth_service: Arc<dyn AuthorizationService>,
    pub token_service: Arc<dyn TokenService>,

    #[cfg(feature = "main")]
    pub git_http_service: Arc<dyn GitHttpService>,
    #[cfg(feature = "main")]
    pub user_service: Arc<dyn UserService>,
    #[cfg(feature = "main")]
    pub org_service: Arc<dyn OrganizationService>,
    #[cfg(feature = "main")]
    pub repo_service: Arc<dyn RepositoryService>,
    #[cfg(feature = "main")]
    pub question_service: Arc<dyn QuestionService>,
    #[cfg(feature = "main")]
    pub commit_service: Arc<dyn CommitService>,

    #[cfg(feature = "ci")]
    pub runner_service: Arc<dyn RunnerService>,
    #[cfg(feature = "ci")]
    pub dag_service: Arc<dyn DagService>,
    #[cfg(feature = "ci")]
    pub task_service: Arc<dyn TaskService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let token_repo = TokenRepositoryImpl::new(pool.clone());

        #[cfg(feature = "main")]
        let git_client = Git2Client::new(settings.git_project_root.clone());
        #[cfg(feature = "main")]
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());
        #[cfg(feature = "main")]
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        #[cfg(feature = "main")]
        let user_repo = UserRepositoryImpl::new(pool.clone());
        #[cfg(feature = "main")]
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        #[cfg(feature = "main")]
        let question_repo = QuestionRepositoryImpl::new(pool.clone());
        #[cfg(feature = "main")]
        let commit_repo = CommitRepositoryImpl::new(pool.clone());

        #[cfg(feature = "ci")]
        let runner_repo = RunnerRepositoryImpl::new(pool.clone());
        #[cfg(feature = "ci")]
        let dag_repo = DagRepositoryImpl::new(pool.clone());
        #[cfg(feature = "ci")]
        let task_repo = TaskRepositoryImpl::new(pool.clone());

        Self {
            settings,

            auth_service: Arc::new(AuthorizationServiceImpl::new(
                org_repo.clone(),
                repo_repo.clone(),
                question_repo.clone(),
                user_repo.clone(),
            )),
            token_service: Arc::new(TokenServiceImpl::new(token_repo.clone(), user_repo.clone())),

            #[cfg(feature = "main")]
            user_service: Arc::new(UserServiceImpl::new(user_repo.clone(), repo_repo.clone())),
            #[cfg(feature = "main")]
            org_service: Arc::new(OrganizationServiceImpl::new(
                org_repo.clone(),
                user_repo.clone(),
                repo_repo.clone(),
            )),
            #[cfg(feature = "main")]
            repo_service: Arc::new(RepositoryServiceImpl::new(
                git_client.clone(),
                org_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
            )),
            #[cfg(feature = "main")]
            question_service: Arc::new(QuestionServiceImpl::new(
                question_repo.clone(),
                repo_repo.clone(),
            )),
            #[cfg(feature = "main")]
            commit_service: Arc::new(CommitServiceImpl::new(commit_repo.clone())),
            #[cfg(feature = "main")]
            git_http_service: Arc::new(GitHttpServiceImpl::new(git_http_client.clone())),

            #[cfg(feature = "ci")]
            runner_service: Arc::new(RunnerServiceImpl::new(
                runner_repo.clone(),
                org_repo.clone(),
                token_repo.clone(),
            )),
            #[cfg(feature = "ci")]
            dag_service: Arc::new(DagServiceImpl::new(dag_repo.clone())),
            #[cfg(feature = "ci")]
            task_service: Arc::new(TaskServiceImpl::new(task_repo.clone())),
        }
    }
}
