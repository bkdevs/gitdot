use figment::{Figment, providers::Env};
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default = "default_port")]
    pub port: u16,

    // external services (non-secret)
    pub vercel_oidc_url: String,

    // app secrets
    pub gitdot_public_key: String,

    // app URLs
    #[serde(default = "default_web_url")]
    pub gitdot_web_url: String,

    // ClickHouse
    #[serde(default = "default_clickhouse_url")]
    pub clickhouse_url: String,
    pub clickhouse_user: String,
    pub clickhouse_password: SecretString,
    pub clickhouse_database: String,
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
    8083
}

fn default_web_url() -> String {
    "http://localhost:3000".into()
}

fn default_clickhouse_url() -> String {
    "http://localhost:8123".into()
}
