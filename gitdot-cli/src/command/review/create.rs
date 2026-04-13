use crate::{
    config::UserConfig,
    git::GitWrapper,
    util::review::{get_remote_owner_repo, push_for_review},
};

pub async fn create_review(config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    // We are only allowing to open reviews against the default branch.
    // Underlying implementation supports opening reviews against any branch.
    let default_branch = git.default_branch().await?;
    git.pull_rebase(&default_branch).await?;
    let review_number = push_for_review(git, &default_branch, None).await?;

    match review_number {
        Some(n) => {
            let (owner, repo) = get_remote_owner_repo(git).await?;
            let url = format!(
                "{}/{}/{}/reviews/{}",
                config.gitdot_web_url.trim_end_matches('/'),
                owner,
                repo,
                n
            );
            println!("Review created. Finish publishing it at: {}", url);
        }
        None => println!("Review created"),
    }

    Ok(())
}
