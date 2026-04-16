use std::io::{self, Write};

use anyhow::{Context, bail};

use gitdot_api::endpoint::list_user_reviews::ListUserReviewsRequest;

use crate::{
    client::GitdotClient,
    config::UserConfig,
    git::GitWrapper,
    util::review::{ReviewPushResult, get_remote_owner_repo, push_for_review},
};

pub async fn update_review(config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    // TODO: init client with token from store
    let client = GitdotClient::from_user_config(&config);
    let (owner, repo) = get_remote_owner_repo(git).await?;
    let reviews = client
        .list_user_reviews(
            &config.user_name,
            ListUserReviewsRequest {
                status: Some("in_progress".to_string()),
                owner: Some(owner.clone()),
                repo: Some(repo.clone()),
            },
        )
        .await?;

    if reviews.is_empty() {
        bail!("No reviews found for user '{}'", config.user_name);
    }

    let selected = if reviews.len() == 1 {
        0
    } else {
        println!("Your reviews:");
        for (i, review) in reviews.iter().enumerate() {
            println!(
                "  [{}] #{} {} ({}, {})",
                i + 1,
                review.number,
                review.title,
                review.status,
                review.target_branch,
            );
        }

        print!("Select a review [1-{}]: ", reviews.len());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice: usize = input.trim().parse().context("Invalid selection")?;

        if choice < 1 || choice > reviews.len() {
            bail!("Selection out of range");
        }

        choice - 1
    };

    let review = &reviews[selected];
    let default_branch = git.default_branch().await?;
    git.pull_rebase(&default_branch).await?;
    let result = push_for_review(git, &default_branch, Some(review.number)).await?;

    match result {
        Some(ReviewPushResult::Draft { id }) => {
            let url = format!(
                "{}/{}/{}/reviews/drafts/{}",
                config.gitdot_web_url.trim_end_matches('/'),
                owner,
                repo,
                id
            );
            println!("Review updated: {}", url);
        }
        Some(ReviewPushResult::Published { number }) => {
            let url = format!(
                "{}/{}/{}/reviews/{}",
                config.gitdot_web_url.trim_end_matches('/'),
                owner,
                repo,
                number
            );
            println!("Review updated: {}", url);
        }
        None => println!("Review updated"),
    }

    Ok(())
}
