use std::path::PathBuf;

use anyhow::{Context, bail};
use tokio::fs;

use crate::git::GitWrapper;

const REVIEW_BRANCH_FILE: &str = "gdot-review-branch";

pub async fn get_remote_owner_repo(git: &GitWrapper) -> anyhow::Result<(String, String)> {
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

pub async fn save_review_branch(git: &GitWrapper, branch: &str) -> anyhow::Result<()> {
    let path = get_review_branch_path(git).await?;
    fs::write(&path, branch).await?;
    Ok(())
}

pub async fn load_review_branch(git: &GitWrapper) -> anyhow::Result<Option<String>> {
    let path = get_review_branch_path(git).await?;
    match fs::read_to_string(&path).await {
        Ok(branch) => Ok(Some(branch.trim().to_string())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn clear_review_branch(git: &GitWrapper) -> anyhow::Result<()> {
    let path = get_review_branch_path(git).await?;
    let _ = fs::remove_file(&path).await;
    Ok(())
}

pub async fn push_for_review(
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

async fn get_review_branch_path(git: &GitWrapper) -> anyhow::Result<PathBuf> {
    Ok(git.git_dir().await?.join(REVIEW_BRANCH_FILE))
}
