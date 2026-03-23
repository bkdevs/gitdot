mod amend;
mod checkout;
mod create;
mod update;

use anyhow::{Context, bail};
use clap::{Args, Subcommand};

use crate::{config::UserConfig, git::GitWrapper};

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
        let git = GitWrapper::new();
        match self {
            ReviewCommand::New {} => create::create_review(config, &git).await,
            ReviewCommand::Checkout {} => checkout::checkout_review(config, &git).await,
            ReviewCommand::Amend {} => amend::amend_review(config, &git).await,
            ReviewCommand::Update {} => update::update_review(config, &git).await,
        }
    }
}

async fn get_remote_owner_repo(git: &GitWrapper) -> anyhow::Result<(String, String)> {
    let url = git.remote_url("origin").await?;

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

async fn push_for_review(
    git: &GitWrapper,
    branch: &str,
    review_number: Option<i32>,
) -> anyhow::Result<Option<String>> {
    let refspec = match review_number {
        Some(number) => format!("HEAD:refs/for/{}/{}", branch, number),
        None => format!("HEAD:refs/for/{}", branch),
    };

    let stderr = git.push_refspec(&refspec).await?;

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
