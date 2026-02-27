use async_trait::async_trait;
use google_cloud_secretmanager_v1::client::SecretManagerService;

use crate::error::SecretError;

const DATABASE_URL_SECRET_NAME: &str = "database-url";
const GITHUB_APP_ID_SECRET_NAME: &str = "github-app-id";
const GITHUB_APP_PRIVATE_KEY_SECRET_NAME: &str = "github-app-private-key";

#[async_trait]
pub trait SecretClient: Send + Sync + Clone + 'static {
    async fn get_database_url(&self) -> Result<String, SecretError>;

    async fn get_github_app_id(&self) -> Result<u64, SecretError>;

    async fn get_github_app_private_key(&self) -> Result<String, SecretError>;
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

#[async_trait]
impl SecretClient for GoogleSecretClient {
    async fn get_database_url(&self) -> Result<String, SecretError> {
        self.access_secret(DATABASE_URL_SECRET_NAME).await
    }

    async fn get_github_app_id(&self) -> Result<u64, SecretError> {
        let value = self.access_secret(GITHUB_APP_ID_SECRET_NAME).await?;
        value.trim().parse::<u64>().map_err(|e| {
            SecretError::ParseError(format!(
                "{} is not a valid u64: {e}",
                GITHUB_APP_ID_SECRET_NAME
            ))
        })
    }

    async fn get_github_app_private_key(&self) -> Result<String, SecretError> {
        self.access_secret(GITHUB_APP_PRIVATE_KEY_SECRET_NAME).await
    }
}
