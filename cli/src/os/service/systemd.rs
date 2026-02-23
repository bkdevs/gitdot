use anyhow::Context;

use crate::util::run_command;

use super::{Service, ServiceManager};

const SERVICE_NAME: &str = "gitdot-runner";
const UNIT_PATH: &str = "/etc/systemd/user/gitdot-runner.service";

impl Service for ServiceManager {
    fn install(&self) -> anyhow::Result<()> {
        let unit = format!(
            "[Unit]
Description=Gitdot Runner
After=network.target

[Service]
ExecStart={binary} ci run
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

    fn uninstall(&self) -> anyhow::Result<()> {
        // Best-effort: ignore errors if service is not installed/running
        let _ = run_command(
            "sudo",
            &["-u", &self.run_as_user, "systemctl", "--user", "stop", SERVICE_NAME],
        );
        let _ = run_command(
            "sudo",
            &["-u", &self.run_as_user, "systemctl", "--user", "disable", SERVICE_NAME],
        );
        let _ = std::fs::remove_file(UNIT_PATH);
        let _ = run_command(
            "sudo",
            &["-u", &self.run_as_user, "systemctl", "--user", "daemon-reload"],
        );
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
