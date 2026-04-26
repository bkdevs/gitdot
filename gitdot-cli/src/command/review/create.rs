use crate::{
    client::GitdotClient,
    config::UserConfig,
    git::GitWrapper,
    store::GitCredentialStore,
    util::review::{get_remote_owner_repo, push_for_review},
};

pub async fn create_review(
    config: UserConfig,
    git: &GitWrapper,
    message: Option<String>,
) -> anyhow::Result<()> {
    let default_branch = git.default_branch().await?;
    git.pull_rebase(&default_branch).await?;
    let result = push_for_review(git, &default_branch, None).await?;

    match result {
        Some(n) => {
            let (owner, repo) = get_remote_owner_repo(git).await?;

            if let Some(msg) = message {
                let (title, description) = parse_message(&msg);
                let token = GitCredentialStore::get(&config.gitdot_server_url, &config.user_name)?;
                let client = GitdotClient::from_user_config(&config).with_token(token);
                client
                    .update_review(
                        &owner,
                        &repo,
                        n,
                        Some(title),
                        if description.is_empty() {
                            None
                        } else {
                            Some(description)
                        },
                    )
                    .await?;
            }

            let url = format!(
                "{}/{}/{}/reviews/{}",
                config.gitdot_web_url.trim_end_matches('/'),
                owner,
                repo,
                n
            );
            println!("Review created: {}", url);
        }
        None => println!("Review created"),
    }

    Ok(())
}

fn parse_message(message: &str) -> (String, String) {
    let message = message.replace("\\n", "\n");
    match message.splitn(2, '\n').collect::<Vec<_>>().as_slice() {
        [title] => (title.trim().to_string(), String::new()),
        [title, rest] => (title.trim().to_string(), rest.trim().to_string()),
        _ => (String::new(), String::new()),
    }
}
