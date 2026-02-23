use super::user_exists::user_exists;
use crate::util::run_command;
use std::process::Command;

pub fn create_user(username: &str) -> anyhow::Result<()> {
    if user_exists(username) {
        return Ok(());
    }

    println!("Creating system user '{username}'...");

    #[cfg(target_os = "macos")]
    {
        let uid = find_free_uid_macos()?;
        let uid_str = uid.to_string();
        let user_path = format!("/Users/{username}");
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
                username,
            ],
        )?;
    }

    println!("System user '{username}' created.");
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
