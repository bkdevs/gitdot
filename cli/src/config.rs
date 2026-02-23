use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[cfg(feature = "main")]
mod profile;
#[cfg(feature = "main")]
pub use profile::ProfileConfig;

#[cfg(feature = "ci")]
mod ci;
#[cfg(feature = "ci")]
pub use ci::CiConfig;

const CONFIG_DIR_NAME: &str = "gitdot";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_gitdot_server_url")]
    pub gitdot_server_url: String,

    #[cfg(feature = "main")]
    #[serde(default)]
    pub profile: ProfileConfig,

    #[cfg(feature = "ci")]
    #[serde(default)]
    pub ci: CiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gitdot_server_url: default_gitdot_server_url(),
            #[cfg(feature = "main")]
            profile: ProfileConfig::default(),
            #[cfg(feature = "ci")]
            ci: CiConfig::default(),
        }
    }
}

fn default_gitdot_server_url() -> String {
    "https://api.gitdot.io".to_string()
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

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let base_dir = dirs_next::config_dir().context("Could not determine config directory")?;
        let config_dir = base_dir.join(CONFIG_DIR_NAME);
        Ok(config_dir.join(CONFIG_FILE_NAME))
    }
}
