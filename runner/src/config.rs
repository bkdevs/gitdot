use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::executor::ExecutorType;

const CONFIG_DIR_NAME: &str = "gitdot-runner";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_gitdot_server_url")]
    pub gitdot_server_url: String,
    pub runner_token: Option<String>,
    #[serde(default = "default_run_as_user")]
    pub run_as_user: String,
    #[serde(default)]
    pub executor: ExecutorType,
}

fn default_gitdot_server_url() -> String {
    "https://api.gitdot.io".to_string()
}

fn default_run_as_user() -> String {
    "gitdot-runner".to_string()
}

impl Config {
    pub async fn load() -> anyhow::Result<Self> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = tokio::fs::read_to_string(&config_path)
            .await
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        Ok(config)
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        // TODO: this needs to request sudo permissions to write to /etc/ on linux
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }
        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        tokio::fs::write(&config_path, contents)
            .await
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let config_dir = match std::env::consts::OS {
            "macos" => {
                let home = std::env::var("HOME").context("HOME environment variable not set")?;
                PathBuf::from(home).join(".config").join(CONFIG_DIR_NAME)
            }
            _ => PathBuf::from("/etc").join(CONFIG_DIR_NAME),
        };
        Ok(config_dir.join(CONFIG_FILE_NAME))
    }
}
