use async_trait::async_trait;
use octocrab::models::{AppId, Installation, InstallationId, InstallationRepositories};
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::error::GitHubError;

#[async_trait]
pub trait GitHubClient: Send + Sync + Clone + 'static {
    // OAuth operations
    fn get_authorization_url(&self, state: &str) -> String;
    async fn exchange_code(&self, code: &str) -> Result<String, GitHubError>;
    async fn get_user_email(&self, access_token: &str) -> Result<String, GitHubError>;

    // GitHub App operations
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
    app_client: octocrab::Octocrab,
    oauth_client: octocrab::Octocrab,
    client_id: String,
    client_secret: String,
}

impl OctocrabClient {
    pub fn new(app_id: u64, private_key: String, client_id: String, client_secret: String) -> Self {
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())
            .expect("Invalid RSA private key PEM");

        let app_client = octocrab::Octocrab::builder()
            .app(AppId(app_id), key)
            .build()
            .expect("Failed to build GitHub App client");

        let oauth_client = octocrab::Octocrab::builder()
            .base_uri("https://github.com")
            .expect("Invalid base URI")
            .add_header(http::header::ACCEPT, "application/json".to_string())
            .build()
            .expect("Failed to build GitHub OAuth client");

        Self {
            app_client,
            oauth_client,
            client_id,
            client_secret,
        }
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

    async fn exchange_code(&self, code: &str) -> Result<String, GitHubError> {
        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
        }

        let response: TokenResponse = self
            .oauth_client
            .post(
                "/login/oauth/access_token",
                Some(&serde_json::json!({
                    "client_id": self.client_id,
                    "client_secret": self.client_secret,
                    "code": code,
                })),
            )
            .await?;

        Ok(response.access_token)
    }

    async fn get_user_email(&self, access_token: &str) -> Result<String, GitHubError> {
        #[derive(Deserialize)]
        struct GitHubEmail {
            email: String,
            primary: bool,
            verified: bool,
        }

        let crab = octocrab::Octocrab::builder()
            .personal_token(access_token.to_string())
            .build()?;

        let emails: Vec<GitHubEmail> = crab.get("/user/emails", None::<&()>).await?;
        emails
            .into_iter()
            .find(|e| e.primary && e.verified)
            .map(|e| e.email)
            .ok_or_else(|| GitHubError::Other("No verified primary email found".to_string()))
    }

    async fn get_installation(&self, installation_id: u64) -> Result<Installation, GitHubError> {
        let installation = self
            .app_client
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
            .app_client
            .installation_and_token(InstallationId(installation_id))
            .await?;

        Ok(token.expose_secret().to_string())
    }

    async fn list_installation_repositories(
        &self,
        installation_id: u64,
    ) -> Result<InstallationRepositories, GitHubError> {
        let client = self
            .app_client
            .installation(InstallationId(installation_id))?;
        let repositories = client
            .get("/installation/repositories", None::<&()>)
            .await?;

        Ok(repositories)
    }
}
