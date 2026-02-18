use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    repository::{
        CodeRepositoryImpl, OrganizationRepositoryImpl, QuestionRepositoryImpl,
        RepositoryRepositoryImpl, TokenRepositoryImpl, UserRepositoryImpl,
    },
    service::{
        AuthorizationService, AuthorizationServiceImpl, OAuthService, OAuthServiceImpl,
        OrganizationService, OrganizationServiceImpl, UserService, UserServiceImpl,
    },
};

cfg_use!("main", {
    use gitdot_core::{
        client::{DifftClient, Git2Client, GitHttpClientImpl},
        repository::CommitRepositoryImpl,
        service::{
            CommitService, CommitServiceImpl, GitHttpService, GitHttpServiceImpl, QuestionService,
            QuestionServiceImpl, RepositoryService, RepositoryServiceImpl,
        },
    };
});

cfg_use!("ci", {
    use gitdot_core::{
        repository::{DagRepositoryImpl, RunnerRepositoryImpl, TaskRepositoryImpl},
        service::{
            DagService, DagServiceImpl, RunnerService, RunnerServiceImpl, TaskService,
            TaskServiceImpl,
        },
    };
});

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub oauth_service: Arc<dyn OAuthService>,
    pub auth_service: Arc<dyn AuthorizationService>,

    pub user_service: Arc<dyn UserService>,
    pub org_service: Arc<dyn OrganizationService>,

    #[cfg(feature = "main")]
    pub git_http_service: Arc<dyn GitHttpService>,
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
        let code_repo = CodeRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let question_repo = QuestionRepositoryImpl::new(pool.clone());

        #[cfg(feature = "main")]
        let git_client = Git2Client::new(settings.git_project_root.clone());
        #[cfg(feature = "main")]
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());
        #[cfg(feature = "main")]
        let diff_client = DifftClient::new();
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
            oauth_service: Arc::new(OAuthServiceImpl::new(
                code_repo.clone(),
                token_repo.clone(),
                user_repo.clone(),
            )),
            auth_service: Arc::new(AuthorizationServiceImpl::new(
                token_repo.clone(),
                org_repo.clone(),
                repo_repo.clone(),
                question_repo.clone(),
                user_repo.clone(),
            )),
            user_service: Arc::new(UserServiceImpl::new(user_repo.clone(), repo_repo.clone())),
            org_service: Arc::new(OrganizationServiceImpl::new(
                org_repo.clone(),
                user_repo.clone(),
                repo_repo.clone(),
            )),

            #[cfg(feature = "main")]
            repo_service: Arc::new(RepositoryServiceImpl::new(
                git_client.clone(),
                diff_client.clone(),
                org_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
            )),
            #[cfg(feature = "main")]
            git_http_service: Arc::new(GitHttpServiceImpl::new(git_http_client.clone())),
            #[cfg(feature = "main")]
            question_service: Arc::new(QuestionServiceImpl::new(
                question_repo.clone(),
                repo_repo.clone(),
            )),
            #[cfg(feature = "main")]
            commit_service: Arc::new(CommitServiceImpl::new(
                commit_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
                git_client.clone(),
            )),

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
