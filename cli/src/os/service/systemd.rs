use anyhow::Context;

use crate::{config::runner::SYSTEM_USER, util::run_command};

use super::{Service, ServiceManager};

const SERVICE_NAME: &str = "gitdot-runner";
const UNIT_PATH: &str = "/etc/systemd/system/gitdot-runner.service";

impl Service for ServiceManager {
    fn install(&self) -> anyhow::Result<()> {
        let unit = format!(
            "[Unit]
Description=Gitdot Runner
After=network.target

[Service]
ExecStart={binary} ci run
User={user}
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier={name}

[Install]
WantedBy=multi-user.target
",
            binary = self.binary_path,
            user = SYSTEM_USER,
            name = SERVICE_NAME,
        );

        std::fs::write(UNIT_PATH, unit).with_context(|| {
            format!(
                "Failed to write unit file to {} (try running with sudo)",
                UNIT_PATH
            )
        })?;

        run_command("sudo", &["systemctl", "daemon-reload"])
            .context("Failed to reload systemd daemon")?;

        run_command("sudo", &["systemctl", "enable", SERVICE_NAME])
            .context("Failed to enable systemd service")?;

        Ok(())
    }

    fn uninstall(&self) -> anyhow::Result<()> {
        // Best-effort: ignore errors and suppress output (service may not be installed yet)
        let silent = |args: &[&str]| {
            let _ = std::process::Command::new("sudo")
                .args(args)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        };
        silent(&["systemctl", "stop", SERVICE_NAME]);
        silent(&["systemctl", "disable", SERVICE_NAME]);
        let _ = std::fs::remove_file(UNIT_PATH);
        silent(&["systemctl", "daemon-reload"]);
        Ok(())
    }

    fn start(&self) -> anyhow::Result<()> {
        run_command("sudo", &["systemctl", "start", SERVICE_NAME])
            .context("Failed to start systemd service")?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        run_command("sudo", &["systemctl", "stop", SERVICE_NAME])
            .context("Failed to stop systemd service")?;
        Ok(())
    }
}
