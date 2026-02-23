use anyhow::Context;

use crate::{config::runner::SYSTEM_USER, util::run_command};

use super::{Service, ServiceManager};

const LABEL: &str = "io.gitdot.runner";
const PLIST_PATH: &str = "/Library/LaunchDaemons/io.gitdot.runner.plist";
const LOG_PATH: &str = "/Library/Logs/gitdot-runner.log";

impl Service for ServiceManager {
    fn install(&self) -> anyhow::Result<()> {
        let plist = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key><string>{label}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{binary}</string>
        <string>ci</string>
        <string>run</string>
    </array>
    <key>UserName</key><string>{user}</string>
    <key>KeepAlive</key><true/>
    <key>RunAtLoad</key><true/>
    <key>StandardOutPath</key><string>{log}</string>
    <key>StandardErrorPath</key><string>{log}</string>
</dict>
</plist>
"#,
            label = LABEL,
            binary = self.binary_path,
            user = SYSTEM_USER,
            log = LOG_PATH,
        );

        let tmp = std::env::temp_dir().join("io.gitdot.runner.plist");
        std::fs::write(&tmp, plist).context("Failed to write plist to temp file")?;
        let tmp_str = tmp.to_str().context("Temp path is not valid UTF-8")?;

        run_command("sudo", &["cp", tmp_str, PLIST_PATH])
            .with_context(|| format!("Failed to copy plist to {}", PLIST_PATH))?;

        run_command(
            "sudo",
            &["launchctl", "bootstrap", "system", PLIST_PATH],
        )
        .context("Failed to bootstrap launchd daemon")?;

        Ok(())
    }

    fn uninstall(&self) -> anyhow::Result<()> {
        // Best-effort: ignore errors and suppress output (service may not be loaded yet)
        let _ = std::process::Command::new("sudo")
            .args(["launchctl", "bootout", "system", PLIST_PATH])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        let _ = std::process::Command::new("sudo")
            .args(["rm", "-f", PLIST_PATH])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        Ok(())
    }

    fn start(&self) -> anyhow::Result<()> {
        run_command(
            "sudo",
            &["launchctl", "kickstart", &format!("system/{}", LABEL)],
        )
        .context("Failed to start launchd daemon")?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        run_command(
            "sudo",
            &[
                "launchctl",
                "kill",
                "TERM",
                &format!("system/{}", LABEL),
            ],
        )
        .context("Failed to stop launchd daemon")?;
        Ok(())
    }
}
