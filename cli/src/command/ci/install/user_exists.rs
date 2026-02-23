use std::process::Command;

pub fn user_exists(username: &str) -> bool {
    Command::new("id")
        .arg(username)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
