use super::{pull_rebase_default_branch, push_for_review};
use crate::config::UserConfig;

pub async fn create_review(_config: UserConfig) -> anyhow::Result<()> {
    let default_branch = pull_rebase_default_branch().await?;
    let review_url = push_for_review(&default_branch, None).await?;

    match review_url {
        Some(url) => println!("Review created. Finish publishing it at: {}", url),
        None => println!("Review created"),
    }

    Ok(())
}
