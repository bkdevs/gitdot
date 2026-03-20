use super::{pull_rebase_default_branch, push_for_review};
use crate::config::UserConfig;

pub async fn create_review(_config: UserConfig) -> anyhow::Result<()> {
    // We are only allowing to open reviews against the default branch.
    // Underlying implementation supports opening reviews against any branch.
    let default_branch = pull_rebase_default_branch().await?;
    let review_url = push_for_review(&default_branch, None).await?;

    match review_url {
        Some(url) => println!("Review created. Finish publishing it at: {}", url),
        None => println!("Review created"),
    }

    Ok(())
}
