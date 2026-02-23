use anyhow::Context;

use crate::{config::ci::SYSTEM_USER, util::run_command};

use super::{Service, ServiceManager};

const LABEL: &str = "io.gitdot.runner";

impl ServiceManager {
    fn agents_dir(&self) -> String {
        format!("/Users/{}/Library/LaunchAgents", SYSTEM_USER)
    }

    fn plist_path(&self) -> String {
        format!(
            "/Users/{}/Library/LaunchAgents/io.gitdot.runner.plist",
            SYSTEM_USER
        )
    }

    fn log_path(&self) -> String {
        format!("/Users/{}/Library/Logs/gitdot-runner.log", SYSTEM_USER)
    }
}

impl Service for ServiceManager {
    fn install(&self) -> anyhow::Result<()> {
        let plist_path = self.plist_path();
        let log_path = self.log_path();

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
    <key>KeepAlive</key><true/>
    <key>RunAtLoad</key><true/>
    <key>StandardOutPath</key><string>{log}</string>
    <key>StandardErrorPath</key><string>{log}</string>
</dict>
</plist>
"#,
            label = LABEL,
            binary = self.binary_path,
            log = log_path,
        );

        run_command(
            "sudo",
            &["-u", SYSTEM_USER, "mkdir", "-p", &self.agents_dir()],
        )
        .context("Failed to create LaunchAgents directory")?;

        let tmp = std::env::temp_dir().join("io.gitdot.runner.plist");
        std::fs::write(&tmp, plist).context("Failed to write plist to temp file")?;
        let tmp_str = tmp.to_str().context("Temp path is not valid UTF-8")?;

        run_command("sudo", &["-u", SYSTEM_USER, "cp", tmp_str, &plist_path])
            .context("Failed to copy plist to LaunchAgents")?;

        run_command(
            "sudo",
            &["-u", SYSTEM_USER, "launchctl", "load", &plist_path],
        )
        .context("Failed to load launchd agent")?;

        Ok(())
    }

    fn uninstall(&self) -> anyhow::Result<()> {
        let plist_path = self.plist_path();
        // Best-effort: ignore errors if not currently loaded or file doesn't exist
        let _ = run_command(
            "sudo",
            &["-u", SYSTEM_USER, "launchctl", "unload", &plist_path],
        );
        let _ = run_command("sudo", &["-u", SYSTEM_USER, "rm", "-f", &plist_path]);
        Ok(())
    }

    fn start(&self) -> anyhow::Result<()> {
        run_command("launchctl", &["start", LABEL]).context("Failed to start launchd agent")?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        run_command("launchctl", &["stop", LABEL]).context("Failed to stop launchd agent")?;
        Ok(())
    }
}
