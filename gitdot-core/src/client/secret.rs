use async_trait::async_trait;
use google_cloud_secretmanager_v1::client::SecretManagerService;

use crate::error::SecretError;

const DATABASE_URL_NAME: &str = "database-url";
const GITDOT_PUBLIC_KEY_NAME: &str = "gitdot-public-key";
const GITDOT_PRIVATE_KEY_NAME: &str = "gitdot-private-key";
const GITDOT_SLACK_SECRET_NAME: &str = "gitdot-slack-secret";

const GITHUB_APP_ID_NAME: &str = "github-app-id";
const GITHUB_APP_PRIVATE_KEY_NAME: &str = "github-app-private-key";
const GITHUB_CLIENT_ID_NAME: &str = "github-client-id";
const GITHUB_CLIENT_SECRET_NAME: &str = "github-client-secret";

const CLOUDFLARE_ACCOUNT_ID_NAME: &str = "cloudflare-account-id";
const CLOUDFLARE_R2_ACCESS_KEY_ID_NAME: &str = "cloudflare-r2-access-key-id";
const CLOUDFLARE_R2_SECRET_ACCESS_KEY_NAME: &str = "cloudflare-r2-secret-access-key";
const CLOUDFLARE_R2_BUCKET_NAME_NAME: &str = "cloudflare-r2-bucket-name";

const RESEND_API_KEY_NAME: &str = "resend-api-key";

#[async_trait]
pub trait SecretClient: Send + Sync + Clone + 'static {
    async fn get_database_url(&self) -> Result<String, SecretError>;
    async fn get_gitdot_public_key(&self) -> Result<String, SecretError>;
    async fn get_gitdot_private_key(&self) -> Result<String, SecretError>;
    async fn get_gitdot_slack_secret(&self) -> Result<String, SecretError>;

    async fn get_github_app_id(&self) -> Result<u64, SecretError>;
    async fn get_github_app_private_key(&self) -> Result<String, SecretError>;
    async fn get_github_client_id(&self) -> Result<String, SecretError>;
    async fn get_github_client_secret(&self) -> Result<String, SecretError>;

    async fn get_cloudflare_account_id(&self) -> Result<String, SecretError>;
    async fn get_cloudflare_r2_bucket_name(&self) -> Result<String, SecretError>;
    async fn get_cloudflare_r2_access_key_id(&self) -> Result<String, SecretError>;
    async fn get_cloudflare_r2_secret_access_key(&self) -> Result<String, SecretError>;

    async fn get_resend_api_key(&self) -> Result<String, SecretError>;
}

#[derive(Clone)]
pub struct GoogleSecretClient {
    client: SecretManagerService,
    project_id: String,
}

impl GoogleSecretClient {
    pub async fn new(project_id: Option<String>) -> Result<Self, SecretError> {
        let client = SecretManagerService::builder().build().await?;
        let project_id = match project_id {
            Some(id) => id,
            None => google_cloud_metadata::project_id().await,
        };
        Ok(Self { client, project_id })
    }

    async fn access_secret(&self, secret_name: &str) -> Result<String, SecretError> {
        let name = format!(
            "projects/{}/secrets/{}/versions/latest",
            self.project_id, secret_name
        );

        let response = self
            .client
            .access_secret_version()
            .set_name(&name)
            .send()
            .await?;

        let payload = response
            .payload
            .ok_or_else(|| SecretError::MissingPayload(secret_name.to_string()))?;

        Ok(String::from_utf8(payload.data.to_vec())?)
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SecretClient for GoogleSecretClient {
    async fn get_database_url(&self) -> Result<String, SecretError> {
        self.access_secret(DATABASE_URL_NAME).await
    }

    async fn get_gitdot_public_key(&self) -> Result<String, SecretError> {
        self.access_secret(GITDOT_PUBLIC_KEY_NAME).await
    }

    async fn get_gitdot_private_key(&self) -> Result<String, SecretError> {
        self.access_secret(GITDOT_PRIVATE_KEY_NAME).await
    }

    async fn get_gitdot_slack_secret(&self) -> Result<String, SecretError> {
        self.access_secret(GITDOT_SLACK_SECRET_NAME).await
    }

    async fn get_github_app_id(&self) -> Result<u64, SecretError> {
        let value = self.access_secret(GITHUB_APP_ID_NAME).await?;
        value.trim().parse::<u64>().map_err(|e| {
            SecretError::ParseError(format!("{} is not a valid u64: {e}", GITHUB_APP_ID_NAME))
        })
    }

    async fn get_github_app_private_key(&self) -> Result<String, SecretError> {
        self.access_secret(GITHUB_APP_PRIVATE_KEY_NAME).await
    }

    async fn get_github_client_id(&self) -> Result<String, SecretError> {
        self.access_secret(GITHUB_CLIENT_ID_NAME).await
    }

    async fn get_github_client_secret(&self) -> Result<String, SecretError> {
        self.access_secret(GITHUB_CLIENT_SECRET_NAME).await
    }

    async fn get_cloudflare_account_id(&self) -> Result<String, SecretError> {
        self.access_secret(CLOUDFLARE_ACCOUNT_ID_NAME).await
    }

    async fn get_cloudflare_r2_access_key_id(&self) -> Result<String, SecretError> {
        self.access_secret(CLOUDFLARE_R2_ACCESS_KEY_ID_NAME).await
    }

    async fn get_cloudflare_r2_bucket_name(&self) -> Result<String, SecretError> {
        self.access_secret(CLOUDFLARE_R2_BUCKET_NAME_NAME).await
    }

    async fn get_cloudflare_r2_secret_access_key(&self) -> Result<String, SecretError> {
        self.access_secret(CLOUDFLARE_R2_SECRET_ACCESS_KEY_NAME)
            .await
    }

    async fn get_resend_api_key(&self) -> Result<String, SecretError> {
        self.access_secret(RESEND_API_KEY_NAME).await
    }
}
