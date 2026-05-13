use figment::{Figment, providers::Env};
use serde::{Deserialize, Deserializer};

use gitdot_core::client::KafkaAuthMode;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    // infra
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_git_project_root")]
    pub git_project_root: String,
    pub database_url: String,

    // external services (non-secret)
    pub s2_server_url: String,
    pub vercel_oidc_url: String,

    // app secrets
    pub gitdot_public_key: String,
    pub gitdot_private_key: String,
    pub gitdot_slack_secret: String,
    pub gitdot_github_secret: String,

    // app URLs
    #[serde(default = "default_web_url")]
    pub gitdot_web_url: String,
    #[serde(default = "default_slack_bot_url")]
    pub gitdot_slack_bot_server_url: String,

    // kafka
    #[serde(default = "default_kafka_bootstrap_servers")]
    pub kafka_bootstrap_servers: String,
    #[serde(default, deserialize_with = "deserialize_kafka_auth")]
    pub kafka_auth: KafkaAuthMode,

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
    8080
}

fn default_git_project_root() -> String {
    "/srv/git".into()
}

fn default_web_url() -> String {
    "http://localhost:3000".into()
}

fn default_slack_bot_url() -> String {
    "http://localhost:3001".into()
}

fn default_kafka_bootstrap_servers() -> String {
    "localhost:9092".into()
}

fn deserialize_kafka_auth<'de, D>(deserializer: D) -> Result<KafkaAuthMode, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(KafkaAuthMode::from_env_str(&s))
}
