use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::executor::ExecutorType;

pub const SYSTEM_USER: &str = "gitdot";

const RUNNER_CONFIG_PATH: &str = "/etc/gitdot/runner.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    pub runner_token: Option<String>,

    #[serde(default)]
    pub executor: ExecutorType,

    #[serde(default = "default_gitdot_server_url")]
    pub gitdot_server_url: String,
}

fn default_gitdot_server_url() -> String {
    "https://api.gitdot.io".to_string()
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            runner_token: None,
            executor: ExecutorType::default(),
            gitdot_server_url: default_gitdot_server_url(),
        }
    }
}

impl RunnerConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path = std::path::Path::new(RUNNER_CONFIG_PATH);

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read runner config: {}", RUNNER_CONFIG_PATH))?;

        let config: RunnerConfig = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse runner config: {}", RUNNER_CONFIG_PATH))?;

        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self).context("Failed to serialize runner config")?;

        let tmp = std::env::temp_dir().join("gitdot-runner.toml");
        std::fs::write(&tmp, &contents).context("Failed to write runner config to temp file")?;
        let tmp_str = tmp.to_str().context("Temp path is not valid UTF-8")?;

        crate::util::run_command("sudo", &["mkdir", "-p", "/etc/gitdot"])
            .context("Failed to create /etc/gitdot directory")?;
        crate::util::run_command("sudo", &["cp", tmp_str, RUNNER_CONFIG_PATH])
            .context("Failed to install runner config to /etc/gitdot/runner.toml")?;

        Ok(())
    }
}
