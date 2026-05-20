use std::sync::Arc;

use axum::extract::FromRef;
use jsonwebtoken::jwk::JwkSet;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use gitdot_core::{
    client::{
        Git2Client, GitHttpClientImpl, ImageClientImpl, KafkaClientImpl, OctocrabClient,
        R2ClientImpl, S2ClientImpl, SlackBotClientImpl, TokenClientImpl,
    },
    repository::{
        BuildRepositoryImpl, CommitRepositoryImpl, GitHubRepositoryImpl, MigrationRepositoryImpl,
        OrganizationRepositoryImpl, QuestionRepositoryImpl, RepositoryRepositoryImpl,
        ReviewRepositoryImpl, RunnerRepositoryImpl, SlackWebhookRepositoryImpl, TaskRepositoryImpl,
        TokenRepositoryImpl, UserRepositoryImpl, WebhookRepositoryImpl,
    },
    service::{
        AuthorizationService, AuthorizationServiceImpl, BuildService, BuildServiceImpl,
        CommitService, CommitServiceImpl, EventService, EventServiceImpl, GitHttpService,
        GitHttpServiceImpl, GithubWebhookService, GithubWebhookServiceImpl, MigrationService,
        MigrationServiceImpl, OrganizationService, OrganizationServiceImpl, QuestionService,
        QuestionServiceImpl, RepositoryService, RepositoryServiceImpl, ReviewService,
        ReviewServiceImpl, RunnerService, RunnerServiceImpl, SlackWebhookService,
        SlackWebhookServiceImpl, TaskService, TaskServiceImpl, TokenService, TokenServiceImpl,
        UserService, UserServiceImpl, WebhookService, WebhookServiceImpl,
    },
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    // auth + authz
    pub token_service: Arc<dyn TokenService>,
    pub authorization_service: Arc<dyn AuthorizationService>,

    // core services
    pub user_service: Arc<dyn UserService>,
    pub org_service: Arc<dyn OrganizationService>,
    pub git_http_service: Arc<dyn GitHttpService>,
    pub repo_service: Arc<dyn RepositoryService>,
    pub question_service: Arc<dyn QuestionService>,
    pub review_service: Arc<dyn ReviewService>,
    pub commit_service: Arc<dyn CommitService>,

    // migration services
    pub migration_service: Arc<dyn MigrationService>,

    // webhook services
    pub webhook_service: Arc<dyn WebhookService>,
    pub slack_webhook_service: Arc<dyn SlackWebhookService>,
    pub github_webhook_service: Arc<dyn GithubWebhookService>,
    pub event_service: Arc<dyn EventService>,

    // ci services
    pub build_service: Arc<dyn BuildService>,
    pub runner_service: Arc<dyn RunnerService>,
    pub task_service: Arc<dyn TaskService>,

    pub vercel_jwks: Arc<JwkSet>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>, pool: PgPool) -> anyhow::Result<Self> {
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let org_repo = OrganizationRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let question_repo = QuestionRepositoryImpl::new(pool.clone());
        let review_repo = ReviewRepositoryImpl::new(pool.clone());
        let commit_repo = CommitRepositoryImpl::new(pool.clone());
        let github_repo = GitHubRepositoryImpl::new(pool.clone());
        let migration_repo = MigrationRepositoryImpl::new(pool.clone());
        let webhook_repo = WebhookRepositoryImpl::new(pool.clone());
        let build_repo = BuildRepositoryImpl::new(pool.clone());
        let runner_repo = RunnerRepositoryImpl::new(pool.clone());
        let task_repo = TaskRepositoryImpl::new(pool.clone());
        let slack_webhook_repo = SlackWebhookRepositoryImpl::new(pool.clone());

        let git_client = Git2Client::new(settings.git_project_root.clone());
        let git_http_client = GitHttpClientImpl::new(settings.git_project_root.clone());
        let github_client = OctocrabClient::new(
            settings.github_app_id,
            settings.github_app_private_key.expose_secret().to_string(),
            settings.github_client_id.clone(),
            settings.github_client_secret.expose_secret().to_string(),
            settings.gitdot_github_secret.expose_secret().to_string(),
        );
        let s2_client = S2ClientImpl::new(
            &settings.s2_server_url,
            settings.gitdot_private_key.expose_secret().to_string(),
        );
        let token_client =
            TokenClientImpl::new(settings.gitdot_private_key.expose_secret().to_string());
        let slack_bot_client = SlackBotClientImpl::new(
            settings.gitdot_slack_bot_server_url.clone(),
            settings.gitdot_slack_secret.expose_secret().to_string(),
        );
        let kafka_client =
            KafkaClientImpl::new(&settings.kafka_bootstrap_servers, settings.kafka_auth).await?;
        let image_client = ImageClientImpl::new();
        let r2_client = R2ClientImpl::new(
            settings.cloudflare_account_id.clone(),
            settings.cloudflare_r2_bucket_name.clone(),
            settings.cloudflare_r2_access_key_id.clone(),
            settings
                .cloudflare_r2_secret_access_key
                .expose_secret()
                .to_string(),
        )
        .await;

        let vercel_jwks = {
            let jwks_url = format!("{}/.well-known/jwks", settings.vercel_oidc_url);
            reqwest::get(&jwks_url).await?.json::<JwkSet>().await?
        };

        Ok(Self {
            settings,
            token_service: Arc::new(TokenServiceImpl::new(
                token_repo.clone(),
                github_client.clone(),
                slack_bot_client.clone(),
                token_client.clone(),
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
                review_repo.clone(),
                commit_repo.clone(),
                image_client.clone(),
                r2_client.clone(),
            )),
            org_service: Arc::new(OrganizationServiceImpl::new(
                org_repo.clone(),
                user_repo.clone(),
                repo_repo.clone(),
                image_client.clone(),
                r2_client.clone(),
            )),

            repo_service: Arc::new(RepositoryServiceImpl::new(
                git_client.clone(),
                org_repo.clone(),
                repo_repo.clone(),
                commit_repo.clone(),
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
            )),
            commit_service: Arc::new(CommitServiceImpl::new(
                commit_repo.clone(),
                repo_repo.clone(),
                user_repo.clone(),
                git_client.clone(),
            )),
            migration_service: Arc::new(MigrationServiceImpl::new(
                git_client.clone(),
                github_client.clone(),
                repo_repo.clone(),
                migration_repo.clone(),
                org_repo.clone(),
                github_repo.clone(),
            )),
            webhook_service: Arc::new(WebhookServiceImpl::new(
                webhook_repo.clone(),
                repo_repo.clone(),
            )),
            slack_webhook_service: Arc::new(SlackWebhookServiceImpl::new(
                slack_webhook_repo.clone(),
                repo_repo.clone(),
                slack_bot_client.clone(),
            )),
            github_webhook_service: Arc::new(GithubWebhookServiceImpl::new(
                repo_repo.clone(),
                migration_repo.clone(),
                github_repo.clone(),
                git_client.clone(),
                github_client.clone(),
            )),
            event_service: Arc::new(EventServiceImpl::new(
                user_repo.clone(),
                git_client.clone(),
                kafka_client.clone(),
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
                token_client.clone(),
            )),
            task_service: Arc::new(TaskServiceImpl::new(
                task_repo.clone(),
                runner_repo.clone(),
                repo_repo.clone(),
            )),

            vercel_jwks: Arc::new(vercel_jwks),
        })
    }
}
