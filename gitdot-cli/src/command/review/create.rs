use crate::{config::UserConfig, git::GitWrapper, util::review::push_for_review};

pub async fn create_review(_config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    // We are only allowing to open reviews against the default branch.
    // Underlying implementation supports opening reviews against any branch.
    let default_branch = git.default_branch().await?;
    git.pull_rebase(&default_branch).await?;
    let review_url = push_for_review(git, &default_branch, None).await?;

    match review_url {
        Some(url) => println!("Review created. Finish publishing it at: {}", url),
        None => println!("Review created"),
    }

    Ok(())
}
