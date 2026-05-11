use figment::{Figment, providers::Env};
use serde::{Deserialize, Deserializer};

use gitdot_core::client::KafkaAuthMode;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    // infra
    pub database_url: String,

    // app secrets
    pub gitdot_slack_secret: String,

    // app URLs
    #[serde(default = "default_slack_bot_url")]
    pub gitdot_slack_bot_server_url: String,

    // kafka
    #[serde(default = "default_kafka_bootstrap_servers")]
    pub kafka_bootstrap_servers: String,
    #[serde(default = "default_kafka_consumer_group_id")]
    pub kafka_consumer_group_id: String,
    #[serde(default, deserialize_with = "deserialize_kafka_auth")]
    pub kafka_auth: KafkaAuthMode,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Figment::new().merge(Env::raw()).extract()?)
    }
}

fn default_slack_bot_url() -> String {
    "http://localhost:3001".into()
}

fn default_kafka_bootstrap_servers() -> String {
    "localhost:9092".into()
}

fn default_kafka_consumer_group_id() -> String {
    "gitdot-consumer".into()
}

fn deserialize_kafka_auth<'de, D>(deserializer: D) -> Result<KafkaAuthMode, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(KafkaAuthMode::from_env_str(&s))
}
