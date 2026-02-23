use anyhow::Context;

use crate::util::run_command;

use super::Service;

const SERVICE_NAME: &str = "gitdot-runner";
const UNIT_PATH: &str = "/etc/systemd/user/gitdot-runner.service";

pub struct ServiceManager {
    binary_path: String,
    run_as_user: String,
}

impl ServiceManager {
    pub fn new(run_as_user: String) -> anyhow::Result<Self> {
        let binary_path = std::env::current_exe()
            .context("Failed to determine current executable path")?
            .to_str()
            .context("Executable path is not valid UTF-8")?
            .to_string();
        Ok(Self {
            binary_path,
            run_as_user,
        })
    }
}

impl Service for ServiceManager {
    fn install(&self) -> anyhow::Result<()> {
        let unit = format!(
            "[Unit]
Description=Gitdot Runner
After=network.target

[Service]
ExecStart={binary} run
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier={name}

[Install]
WantedBy=default.target
",
            binary = self.binary_path,
            name = SERVICE_NAME,
        );

        // /etc/systemd/user/ is admin-managed; install runs as root so write directly
        std::fs::write(UNIT_PATH, unit).with_context(|| {
            format!(
                "Failed to write unit file to {} (try running with sudo)",
                UNIT_PATH
            )
        })?;

        // Enable linger so the user's systemd instance starts without an active login session.
        // This also creates /run/user/<uid>/ which the subsequent systemctl --user calls need.
        run_command("loginctl", &["enable-linger", &self.run_as_user])
            .context("Failed to enable linger for user")?;

        run_command(
            "sudo",
            &[
                "-u",
                &self.run_as_user,
                "systemctl",
                "--user",
                "daemon-reload",
            ],
        )
        .context("Failed to reload systemd user daemon")?;

        run_command(
            "sudo",
            &[
                "-u",
                &self.run_as_user,
                "systemctl",
                "--user",
                "enable",
                SERVICE_NAME,
            ],
        )
        .context("Failed to enable systemd user service")?;

        Ok(())
    }

    fn start(&self) -> anyhow::Result<()> {
        run_command("systemctl", &["--user", "start", SERVICE_NAME])
            .context("Failed to start systemd user service")?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        run_command("systemctl", &["--user", "stop", SERVICE_NAME])
            .context("Failed to stop systemd user service")?;
        Ok(())
    }
}
