use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::executor::ExecutorType;

const CONFIG_DIR_NAME: &str = "gitdot";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_gitdot_server_url")]
    pub gitdot_server_url: String,

    #[serde(default)]
    pub profile: ProfileConfig,

    #[serde(default)]
    pub ci: CiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gitdot_server_url: default_gitdot_server_url(),
            profile: ProfileConfig::default(),
            ci: CiConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileConfig {
    #[serde(default)]
    pub user_name: String,

    #[serde(default)]
    pub user_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiConfig {
    pub runner_token: Option<String>,

    #[serde(default = "default_run_as_user")]
    pub run_as_user: String,

    #[serde(default)]
    pub executor: ExecutorType,
}

impl Default for CiConfig {
    fn default() -> Self {
        Self {
            runner_token: None,
            run_as_user: default_run_as_user(),
            executor: ExecutorType::default(),
        }
    }
}

fn default_gitdot_server_url() -> String {
    "https://api.gitdot.io".to_string()
}

fn default_run_as_user() -> String {
    "gitdot-runner".to_string()
}

pub enum AuthStatus {
    LoggedIn { user_name: String },
    LoggedOut,
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

    pub fn get_auth_status(&self) -> AuthStatus {
        if self.profile.user_name.is_empty() {
            AuthStatus::LoggedOut
        } else {
            AuthStatus::LoggedIn {
                user_name: self.profile.user_name.clone(),
            }
        }
    }

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let base_dir = dirs_next::config_dir().context("Could not determine config directory")?;
        let config_dir = base_dir.join(CONFIG_DIR_NAME);
        Ok(config_dir.join(CONFIG_FILE_NAME))
    }
}
