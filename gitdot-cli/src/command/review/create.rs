use std::process::Stdio;

use anyhow::{Context, bail};
use tokio::process::Command;

use crate::config::UserConfig;

pub async fn create_review(_config: UserConfig) -> anyhow::Result<()> {
    let default_branch = get_default_branch().await?;
    let refspec = format!("HEAD:refs/for/{}", default_branch);

    let output = Command::new("git")
        .args(["push", "origin", &refspec])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .context("Failed to run git push")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", stderr.trim());
        bail!("Failed to create review");
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let review_url = stderr.lines().find_map(|line| {
        let trimmed = line.strip_prefix("remote: ")?;
        trimmed
            .split_whitespace()
            .find(|w| w.starts_with("https://"))
    });

    match review_url {
        Some(url) => println!("Review created. Finish publishing it at: {}", url),
        None => println!("Review created"),
    }

    Ok(())
}

async fn get_default_branch() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])
        .output()
        .await?;
    let remote_head = String::from_utf8(output.stdout)?.trim().to_string();
    let default_branch = remote_head
        .rsplit('/')
        .next()
        .unwrap_or(&remote_head)
        .to_string();
    Ok(default_branch)
}
