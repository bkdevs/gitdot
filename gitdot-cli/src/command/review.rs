mod amend;
mod checkout;
mod create;
mod update;

use std::path::PathBuf;

use anyhow::Context;
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
    Create,
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
            ReviewCommand::Create {} => create::create_review(config).await,
            ReviewCommand::Checkout {} => checkout::checkout_review(config).await,
            ReviewCommand::Amend {} => amend::amend_review(config).await,
            ReviewCommand::Update {} => update::update_review(config).await,
        }
    }
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
