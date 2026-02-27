use anyhow::{Error, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::RequestBuilder;

use gitdot_api::{ApiRequest, ApiResource};

const WEB_URL: &str = "https://www.gitdot.io";
const SERVER_URL: &str = "https://api.gitdot.io";

pub struct GitdotClient {
    client: reqwest::Client,
    client_id: String,
    token: Option<String>,
    web_url: String,
    server_url: String,
}

impl GitdotClient {
    pub fn new(client_id: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            client_id: String::from(client_id),
            token: None,
            web_url: WEB_URL.to_string(),
            server_url: SERVER_URL.to_string(),
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

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
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

    pub(crate) async fn get<T, R>(&self, path: String, request: T) -> Result<R, Error>
    where
        T: ApiRequest,
        R: ApiResource,
    {
        let url = format!("{}/{}", self.server_url, path);
        let response = self
            .client
            .get(&url)
            .auth(&self.token)
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
            .auth(&self.token)
            .query(&request)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
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
            .auth(&self.token)
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
            .auth(&self.token)
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
            .auth(&self.token)
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
    fn auth(self, token: &Option<String>) -> Self;
}

impl Auth for RequestBuilder {
    fn auth(self, token: &Option<String>) -> Self {
        match token {
            Some(t) => {
                let encoded = STANDARD.encode(format!("runner:{}", t));
                self.header("Authorization", format!("Basic {}", encoded))
            }
            None => self,
        }
    }
}

pub mod methods;
