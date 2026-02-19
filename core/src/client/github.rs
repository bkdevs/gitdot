use async_trait::async_trait;
use octocrab::models::{AppId, Installation, InstallationId, InstallationRepositories};
use secrecy::ExposeSecret;

use crate::error::GitHubError;

#[async_trait]
pub trait GitHubClient: Send + Sync + Clone + 'static {
    async fn get_installation(&self, installation_id: u64) -> Result<Installation, GitHubError>;

    async fn get_installation_access_token(
        &self,
        installation_id: u64,
    ) -> Result<String, GitHubError>;

    async fn list_installation_repositories(
        &self,
        installation_id: u64,
    ) -> Result<InstallationRepositories, GitHubError>;
}

#[derive(Debug, Clone)]
pub struct OctocrabClient {
    client: octocrab::Octocrab,
}

impl OctocrabClient {
    pub fn new(app_id: u64, private_key: String) -> Self {
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())
            .expect("Invalid RSA private key PEM");

        let client = octocrab::Octocrab::builder()
            .app(AppId(app_id), key)
            .build()
            .expect("Failed to build Octocrab client");

        Self { client: client }
    }
}

#[async_trait]
impl GitHubClient for OctocrabClient {
    async fn get_installation(&self, installation_id: u64) -> Result<Installation, GitHubError> {
        let installation = self
            .client
            .apps()
            .installation(InstallationId(installation_id))
            .await?;

        Ok(installation)
    }

    async fn get_installation_access_token(
        &self,
        installation_id: u64,
    ) -> Result<String, GitHubError> {
        let (_, token) = self
            .client
            .installation_and_token(InstallationId(installation_id))
            .await?;

        Ok(token.expose_secret().to_string())
    }

    async fn list_installation_repositories(
        &self,
        installation_id: u64,
    ) -> Result<InstallationRepositories, GitHubError> {
        let client = self.client.installation(InstallationId(installation_id))?;
        let repositories = client
            .get("/installation/repositories", None::<&()>)
            .await?;

        Ok(repositories)
    }
}
