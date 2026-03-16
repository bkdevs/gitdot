mod checkout;
mod create;

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

    /// Checkout a diff from the review
    Checkout,
}

impl ReviewCommand {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        match self {
            ReviewCommand::Create {} => create::create_review(config).await,
            ReviewCommand::Checkout {} => checkout::checkout_review(config).await,
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
