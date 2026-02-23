use std::process::Command;

use crate::util::run_command;

use super::service::{Service, ServiceManager};

pub fn install_service(run_as_user: &str) -> anyhow::Result<()> {
    if run_as_user == "gitdot-runner" {
        ensure_gitdot_runner_user()?;
    } else if !user_exists(run_as_user) {
        anyhow::bail!(
            "User '{}' does not exist. Please create the user before running install.",
            run_as_user
        );
    }

    let manager = ServiceManager::new(run_as_user.to_string())?;

    println!("Removing any existing runner service...");
    manager.uninstall()?;

    println!("Installing runner service...");
    manager.install()?;
    println!("Runner installed.");

    Ok(())
}

fn user_exists(username: &str) -> bool {
    Command::new("id")
        .arg(username)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn ensure_gitdot_runner_user() -> anyhow::Result<()> {
    const USERNAME: &str = "gitdot-runner";

    if user_exists(USERNAME) {
        return Ok(());
    }

    println!("Creating system user '{USERNAME}'...");

    #[cfg(target_os = "macos")]
    {
        let uid = find_free_uid_macos()?;
        let uid_str = uid.to_string();
        let user_path = format!("/Users/{USERNAME}");
        let cmds: &[&[&str]] = &[
            &["dscl", ".", "-create", &user_path],
            &["dscl", ".", "-create", &user_path, "UniqueID", &uid_str],
            &["dscl", ".", "-create", &user_path, "PrimaryGroupID", "20"],
            &[
                "dscl",
                ".",
                "-create",
                &user_path,
                "UserShell",
                "/usr/bin/false",
            ],
            &[
                "dscl",
                ".",
                "-create",
                &user_path,
                "RealName",
                "Gitdot Runner",
            ],
            &[
                "dscl",
                ".",
                "-create",
                &user_path,
                "NFSHomeDirectory",
                "/var/empty",
            ],
        ];
        for cmd in cmds {
            run_command("sudo", cmd)?;
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        run_command(
            "sudo",
            &[
                "useradd",
                "--system",
                "--shell",
                "/usr/sbin/nologin",
                "--no-create-home",
                USERNAME,
            ],
        )?;
    }

    println!("System user '{USERNAME}' created.");
    Ok(())
}

#[cfg(target_os = "macos")]
fn find_free_uid_macos() -> anyhow::Result<u32> {
    for uid in 200u32..=400 {
        let out = Command::new("dscl")
            .args([".", "-search", "/Users", "UniqueID", &uid.to_string()])
            .output()?;
        if String::from_utf8_lossy(&out.stdout).trim().is_empty() {
            return Ok(uid);
        }
    }
    anyhow::bail!("No free UID found in range 200-400 for gitdot-runner user");
}
