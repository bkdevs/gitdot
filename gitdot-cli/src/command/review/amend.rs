use anyhow::{Context, bail};
use tokio::{fs, process::Command};

use super::{git_dir, rev_parse};
use crate::config::UserConfig;

pub async fn amend_review(_config: UserConfig) -> anyhow::Result<()> {
    let git_dir = git_dir().await?;
    let branch_file = git_dir.join("gdot-review-branch");

    let branch = fs::read_to_string(&branch_file)
        .await
        .context("No review checkout in progress. Run `gdot review checkout` first.")?;
    let branch = branch.trim();

    let original_hash = rev_parse("HEAD").await?;

    let status = Command::new("git")
        .args(["add", "-A"])
        .status()
        .await
        .context("Failed to run git add")?;
    if !status.success() {
        bail!("Failed to stage changes");
    }

    let status = Command::new("git")
        .args(["commit", "--amend", "--no-edit"])
        .status()
        .await
        .context("Failed to run git commit --amend")?;
    if !status.success() {
        bail!("Failed to amend commit");
    }

    let new_hash = rev_parse("HEAD").await?;
    let status = Command::new("git")
        .args(["rebase", "--onto", &new_hash, &original_hash, branch])
        .status()
        .await
        .context("Failed to run git rebase")?;
    if !status.success() {
        bail!("Rebase failed. Resolve conflicts and run `git rebase --continue`.");
    }

    let _ = fs::remove_file(&branch_file).await;

    println!("Amended and rebased onto {}", branch);

    Ok(())
}
