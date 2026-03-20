mod amend;
mod checkout;
mod create;
mod update;

use std::{path::PathBuf, process::Stdio};

use anyhow::{Context, bail};
use clap::{Args, Subcommand};
use tokio::process::Command;

use crate::config::UserConfig;

#[derive(Args, Debug)]
pub struct ReviewArgs {
    #[command(subcommand)]
    pub command: ReviewCommand,
}

#[derive(Subcommand, Debug)]
pub enum ReviewCommand {
    /// Create a review
    New,

    /// Checkout a commit from the review
    Checkout,

    /// Amend changes into the checked-out commit and rebase
    Amend,

    /// Update an existing review
    Update,
}

impl ReviewCommand {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        match self {
            ReviewCommand::New {} => create::create_review(config).await,
            ReviewCommand::Checkout {} => checkout::checkout_review(config).await,
            ReviewCommand::Amend {} => amend::amend_review(config).await,
            ReviewCommand::Update {} => update::update_review(config).await,
        }
    }
}

async fn get_remote_owner_repo() -> anyhow::Result<(String, String)> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .await
        .context("Failed to get git remote URL")?;

    if !output.status.success() {
        bail!("No 'origin' remote found");
    }

    let url = String::from_utf8(output.stdout)?.trim().to_string();
    let path = if let Some(rest) = url.strip_prefix("git@") {
        rest.split_once(':')
            .map(|(_, path)| path.to_string())
            .context("Invalid SSH remote URL")?
    } else {
        let segments: Vec<&str> = url.trim_end_matches('/').rsplit('/').take(2).collect();
        if segments.len() < 2 {
            bail!("Could not parse owner/repo from remote URL: {}", url);
        }
        format!("{}/{}", segments[1], segments[0])
    };
    let path = path.strip_suffix(".git").unwrap_or(&path);
    let (owner, repo) = path
        .split_once('/')
        .context("Could not parse owner/repo from remote URL")?;

    Ok((owner.to_string(), repo.to_string()))
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

async fn git_dir() -> anyhow::Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .await
        .context("Failed to get git dir")?;
    let dir = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(PathBuf::from(dir))
}

async fn rev_parse(rev: &str) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", rev])
        .output()
        .await
        .context("Failed to run git rev-parse")?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

async fn pull_rebase_default_branch() -> anyhow::Result<String> {
    let default_branch = get_default_branch().await?;

    let status = Command::new("git")
        .args(["pull", "origin", &default_branch, "--rebase"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .context("Failed to run git pull")?;

    if !status.success() {
        bail!("Failed to rebase onto origin/{}", default_branch);
    }

    Ok(default_branch)
}

async fn push_for_review(
    branch: &str,
    review_number: Option<i32>,
) -> anyhow::Result<Option<String>> {
    let refspec = match review_number {
        Some(number) => format!("HEAD:refs/for/{}/{}", branch, number),
        None => format!("HEAD:refs/for/{}", branch),
    };

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
        bail!("Failed to push to {}", refspec);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let review_url = stderr
        .lines()
        .find_map(|line| {
            let trimmed = line.strip_prefix("remote: ")?;
            trimmed
                .split_whitespace()
                .find(|w| w.starts_with("https://"))
        })
        .map(|s| s.to_string());

    Ok(review_url)
}
