use anyhow::{Error, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::RequestBuilder;

use gitdot_api::{ApiRequest, ApiResource};

const WEB_URL: &str = "https://www.gitdot.io";
const SERVER_URL: &str = "https://api.gitdot.io";
const AUTH_SERVER_URL: &str = "https://auth.gitdot.io";

pub enum Credentials {
    Token(String),
    Jwt(String),
}

pub struct GitdotClient {
    client: reqwest::Client,
    client_id: String,
    credentials: Option<Credentials>,
    web_url: String,
    server_url: String,
    auth_server_url: String,
}

impl GitdotClient {
    pub fn new(client_id: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            client_id: String::from(client_id),
            credentials: None,
            web_url: WEB_URL.to_string(),
            server_url: SERVER_URL.to_string(),
            auth_server_url: AUTH_SERVER_URL.to_string(),
        }
    }

    pub fn with_web_url(mut self, web_url: &str) -> Self {
        self.web_url = web_url.to_string();
        self
    }

    pub fn with_server_url(mut self, server_url: &str) -> Self {
        self.server_url = server_url.to_string();
        self
    }

    pub fn with_auth_server_url(mut self, auth_server_url: &str) -> Self {
        self.auth_server_url = auth_server_url.to_string();
        self
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.credentials = Some(Credentials::Token(token));
        self
    }

    pub fn with_jwt(mut self, token: String) -> Self {
        self.credentials = Some(Credentials::Jwt(token));
        self
    }

    #[cfg(feature = "runner")]
    pub fn from_runner_config(config: &crate::config::RunnerConfig) -> Self {
        let mut client = Self::new("gitdot-runner")
            .with_server_url(&config.gitdot_server_url)
            .with_web_url(&config.gitdot_web_url);
        if let Some(token) = &config.runner_token {
            client = client.with_token(token.clone());
        }
        client
    }

    #[cfg(feature = "main")]
    pub fn from_user_config(config: &crate::config::UserConfig) -> Self {
        Self::new("gitdot-cli")
            .with_server_url(&config.gitdot_server_url)
            .with_web_url(&config.gitdot_web_url)
            .with_auth_server_url(&config.gitdot_auth_server_url)
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_web_url(&self) -> &str {
        &self.web_url
    }

    #[allow(dead_code)]
    pub fn get_server_url(&self) -> &str {
        &self.server_url
    }

    pub fn get_auth_server_url(&self) -> &str {
        &self.auth_server_url
    }

    pub(crate) async fn get<T, R>(&self, path: String, request: T) -> Result<R, Error>
    where
        T: ApiRequest,
        R: ApiResource,
    {
        let url = format!("{}/{}", self.server_url, path);
        let response = self
            .client
            .get(&url)
            .auth(&self.credentials)
            .query(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?;

        Ok(response)
    }

    #[allow(dead_code)]
    pub(crate) async fn head<T>(&self, path: String, request: T) -> Result<(), Error>
    where
        T: ApiRequest,
    {
        let url = format!("{}/{}", self.server_url, path);
        self.client
            .head(&url)
            .auth(&self.credentials)
            .query(&request)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub(crate) async fn auth_post<T, R>(&self, path: String, request: T) -> Result<R, Error>
    where
        T: ApiRequest,
        R: ApiResource,
    {
        let url = format!("{}/{}", self.auth_server_url, path);
        let response = self
            .client
            .post(&url)
            .auth(&self.credentials)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?;

        Ok(response)
    }

    pub(crate) async fn post<T, R>(&self, path: String, request: T) -> Result<R, Error>
    where
        T: ApiRequest,
        R: ApiResource,
    {
        let url = format!("{}/{}", self.server_url, path);
        let response = self
            .client
            .post(&url)
            .auth(&self.credentials)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?;

        Ok(response)
    }

    pub(crate) async fn patch<T, R>(&self, path: String, request: T) -> Result<R, Error>
    where
        T: ApiRequest,
        R: ApiResource,
    {
        let url = format!("{}/{}", self.server_url, path);
        let response = self
            .client
            .patch(&url)
            .auth(&self.credentials)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?;

        Ok(response)
    }

    #[allow(dead_code)]
    pub(crate) async fn delete<T, R>(&self, path: String, request: T) -> Result<R, Error>
    where
        T: ApiRequest,
        R: ApiResource,
    {
        let url = format!("{}/{}", self.server_url, path);
        let response = self
            .client
            .delete(&url)
            .auth(&self.credentials)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?;

        Ok(response)
    }
}

trait Auth {
    fn auth(self, auth: &Option<Credentials>) -> Self;
}

impl Auth for RequestBuilder {
    fn auth(self, credentials: &Option<Credentials>) -> Self {
        match credentials {
            Some(Credentials::Token(t)) => {
                let encoded = STANDARD.encode(format!("runner:{}", t));
                self.header("Authorization", format!("Basic {}", encoded))
            }
            Some(Credentials::Jwt(t)) => self.header("Authorization", format!("Bearer {}", t)),
            None => self,
        }
    }
}

pub mod methods;
