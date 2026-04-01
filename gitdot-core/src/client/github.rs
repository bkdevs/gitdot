use async_trait::async_trait;
use octocrab::models::{AppId, Installation, InstallationId, InstallationRepositories};
use secrecy::ExposeSecret;

use crate::error::GitHubError;

#[async_trait]
pub trait GitHubClient: Send + Sync + Clone + 'static {
    // --- OAuth operations ---

    fn get_authorization_url(&self, state: &str) -> String;

    // --- GitHub App operations ---

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
    client_id: String,
}

impl OctocrabClient {
    pub fn new(app_id: u64, private_key: String, client_id: String) -> Self {
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())
            .expect("Invalid RSA private key PEM");

        let client = octocrab::Octocrab::builder()
            .app(AppId(app_id), key)
            .build()
            .expect("Failed to build Octocrab client");

        Self { client, client_id }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl GitHubClient for OctocrabClient {
    fn get_authorization_url(&self, state: &str) -> String {
        let mut url = url::Url::parse("https://github.com/login/oauth/authorize").unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("scope", "user:email")
            .append_pair("state", state);
        url.to_string()
    }

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
