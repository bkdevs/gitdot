use anyhow::Context;
use tokio::fs;

use crate::{config::UserConfig, git::GitWrapper};

pub async fn amend_review(_config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    let git_dir = git.git_dir().await?;
    let branch_file = git_dir.join("gdot-review-branch");

    let branch = fs::read_to_string(&branch_file)
        .await
        .context("No review checkout in progress. Run `gdot review checkout` first.")?;
    let branch = branch.trim();

    let original_hash = git.rev_parse("HEAD").await?;

    git.add_all().await?;
    git.commit_amend_no_edit().await?;

    let new_hash = git.rev_parse("HEAD").await?;
    git.rebase_onto(&new_hash, &original_hash, branch).await?;

    let _ = fs::remove_file(&branch_file).await;

    println!("Amended and rebased onto {}", branch);

    Ok(())
}
