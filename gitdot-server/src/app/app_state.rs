use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    client::{
        DifftClient, Git2Client, GitHttpClientImpl, OctocrabClient, S2ClientImpl, SecretClient,
    },
    repository::{
        BuildRepositoryImpl, CodeRepositoryImpl, CommitRepositoryImpl, GitHubRepositoryImpl,
        MigrationRepositoryImpl, OrganizationRepositoryImpl, QuestionRepositoryImpl,
        RepositoryRepositoryImpl, ReviewRepositoryImpl, RunnerRepositoryImpl, TaskRepositoryImpl,
        TokenRepositoryImpl, UserRepositoryImpl,
    },
    service::{
        AuthenticationService, AuthenticationServiceImpl, AuthorizationService,
        AuthorizationServiceImpl, BuildService, BuildServiceImpl, CommitService, CommitServiceImpl,
        GitHttpService, GitHttpServiceImpl, MigrationService, MigrationServiceImpl, OAuthService,
        OAuthServiceImpl, OrganizationService, OrganizationServiceImpl, QuestionService,
        QuestionServiceImpl, RepositoryService, RepositoryServiceImpl, ReviewService,
        ReviewServiceImpl, RunnerService, RunnerServiceImpl, TaskService, TaskServiceImpl,
        UserService, UserServiceImpl,
    },
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub oauth_service: Arc<dyn OAuthService>,
    pub authentication_service: Arc<dyn AuthenticationService>,
    pub authorization_service: Arc<dyn AuthorizationService>,

    pub user_service: Arc<dyn UserService>,
    pub org_service: Arc<dyn OrganizationService>,

    pub git_http_service: Arc<dyn GitHttpService>,
    pub repo_service: Arc<dyn RepositoryService>,
    pub question_service: Arc<dyn QuestionService>,
    pub review_service: Arc<dyn ReviewService>,
    pub commit_service: Arc<dyn CommitService>,
    pub migration_service: Arc<dyn MigrationService>,

    pub build_service: Arc<dyn BuildService>,
    pub runner_service: Arc<dyn RunnerService>,
    pub task_service: Arc<dyn TaskService>,

    #[cfg(feature = "otel")]
    pub vercel_jwks: Arc<JwkSet>,
}

impl AppState {
    pub async fn new(
        settings: Arc<Settings>,
        pool: PgPool,
        secret_client: impl SecretClient,
    ) -> anyhow::Result<Self> {
        let code_repo = CodeRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let question_repo = QuestionRepositoryImpl::new(pool.clone());
        let review_repo = ReviewRepositoryImpl::new(pool.clone());
        let commit_repo = CommitRepositoryImpl::new(pool.clone());
        let github_repo = GitHubRepositoryImpl::new(pool.clone());
        let migration_repo = MigrationRepositoryImpl::new(pool.clone());
        let build_repo = BuildRepositoryImpl::new(pool.clone());
        let runner_repo = RunnerRepositoryImpl::new(pool.clone());
        let task_repo = TaskRepositoryImpl::new(pool.clone());

        let git_client = Git2Client::new(settings.git_project_root.clone());
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());
        let diff_client = DifftClient::new();
        let github_client = OctocrabClient::new(
            secret_client.get_github_app_id().await?,
            secret_client.get_github_app_private_key().await?,
        );
        let gitdot_private_key = secret_client.get_gitdot_private_key().await?;
        let s2_client = S2ClientImpl::new(&settings.s2_server_url, gitdot_private_key.clone());

        #[cfg(feature = "otel")]
        let vercel_jwks = {
            use jsonwebtoken::jwk::JwkSet;
            let jwks_url = format!("{}/.well-known/jwks", settings.vercel_oidc_url);
            reqwest::get(&jwks_url).await?.json::<JwkSet>().await?
        };

        Ok(Self {
            settings,
            oauth_service: Arc::new(OAuthServiceImpl::new(
                code_repo.clone(),
                token_repo.clone(),
                user_repo.clone(),
            )),
            authentication_service: Arc::new(AuthenticationServiceImpl::new(
                token_repo.clone(),
                gitdot_private_key,
            )),
            authorization_service: Arc::new(AuthorizationServiceImpl::new(
                org_repo.clone(),
                repo_repo.clone(),
                question_repo.clone(),
                user_repo.clone(),
                review_repo.clone(),
            )),
            user_service: Arc::new(UserServiceImpl::new(
                user_repo.clone(),
                repo_repo.clone(),
                org_repo.clone(),
            )),
            org_service: Arc::new(OrganizationServiceImpl::new(
                org_repo.clone(),
                user_repo.clone(),
                repo_repo.clone(),
            )),

            repo_service: Arc::new(RepositoryServiceImpl::new(
                git_client.clone(),
                org_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
            )),
            git_http_service: Arc::new(GitHttpServiceImpl::new(git_http_client.clone())),
            question_service: Arc::new(QuestionServiceImpl::new(
                question_repo.clone(),
                repo_repo.clone(),
            )),
            review_service: Arc::new(ReviewServiceImpl::new(
                review_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
                git_client.clone(),
                diff_client.clone(),
            )),
            commit_service: Arc::new(CommitServiceImpl::new(
                commit_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
                git_client.clone(),
                diff_client.clone(),
            )),
            migration_service: Arc::new(MigrationServiceImpl::new(
                git_client.clone(),
                github_client.clone(),
                repo_repo.clone(),
                migration_repo.clone(),
                org_repo.clone(),
                github_repo.clone(),
            )),
            build_service: Arc::new(BuildServiceImpl::new(
                git_client.clone(),
                s2_client.clone(),
                build_repo.clone(),
                task_repo.clone(),
                repo_repo.clone(),
            )),

            runner_service: Arc::new(RunnerServiceImpl::new(
                runner_repo.clone(),
                org_repo.clone(),
                token_repo.clone(),
            )),
            task_service: Arc::new(TaskServiceImpl::new(
                task_repo.clone(),
                runner_repo.clone(),
                repo_repo.clone(),
            )),

            #[cfg(feature = "otel")]
            vercel_jwks: Arc::new(vercel_jwks),
        })
    }
}
