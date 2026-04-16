use anyhow::bail;

use crate::{
    config::UserConfig,
    git::GitWrapper,
    util::review::{ReviewPushResult, get_remote_owner_repo, push_for_review},
};

pub async fn create_review(config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    // We are only allowing to open reviews against the default branch.
    // Underlying implementation supports opening reviews against any branch.
    let default_branch = git.default_branch().await?;
    git.pull_rebase(&default_branch).await?;
    let result = push_for_review(git, &default_branch, None).await?;

    match result {
        Some(ReviewPushResult::Draft { id }) => {
            let (owner, repo) = get_remote_owner_repo(git).await?;
            let url = format!(
                "{}/{}/{}/reviews/drafts/{}",
                config.gitdot_web_url.trim_end_matches('/'),
                owner,
                repo,
                id
            );
            println!("Review created: {}", url);
        }
        Some(ReviewPushResult::Published { .. }) => {
            bail!("Expected a draft review but got a published review number");
        }
        None => println!("Review created"),
    }

    Ok(())
}
