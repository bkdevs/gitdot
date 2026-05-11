use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    // infra
    #[serde(default = "default_port")]
    pub port: u16,
    pub database_url: String,

    // app secrets
    pub gitdot_public_key: String,
    pub gitdot_private_key: String,
    pub gitdot_slack_secret: String,

    // app urls
    #[serde(default = "default_web_url")]
    pub gitdot_web_url: String,
    #[serde(default = "default_slack_bot_url")]
    pub gitdot_slack_bot_server_url: String,
    #[serde(default = "default_device_url")]
    pub gitdot_oauth_device_verification_url: String,

    // github
    pub github_app_id: u64,
    pub github_app_private_key: String,
    pub github_client_id: String,
    pub github_client_secret: String,

    // cloudflare
    pub cloudflare_account_id: String,
    pub cloudflare_r2_bucket_name: String,
    pub cloudflare_r2_access_key_id: String,
    pub cloudflare_r2_secret_access_key: String,

    // resend
    pub resend_api_key: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Figment::new().merge(Env::raw()).extract()?)
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

fn default_port() -> u16 {
    8082
}

fn default_web_url() -> String {
    "http://localhost:3000".into()
}

fn default_slack_bot_url() -> String {
    "http://localhost:3001".into()
}

fn default_device_url() -> String {
    "http://localhost:3000/oauth/device".into()
}
