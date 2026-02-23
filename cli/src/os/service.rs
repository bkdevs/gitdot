use anyhow::{Context, Result};

pub trait Service {
    fn install(&self) -> Result<()>;
    fn uninstall(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
}

pub struct ServiceManager {
    binary_path: String,
}

impl ServiceManager {
    pub fn new() -> Result<Self> {
        let binary_path = std::env::current_exe()
            .context("Failed to determine current executable path")?
            .to_str()
            .context("Executable path is not valid UTF-8")?
            .to_string();
        Ok(Self { binary_path })
    }
}

#[cfg(target_os = "macos")]
mod launchd;

#[cfg(target_os = "linux")]
mod systemd;
