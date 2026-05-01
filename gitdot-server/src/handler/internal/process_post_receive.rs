use std::env;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use reqwest::Client;
use serde_json::json;

use gitdot_core::{
    dto::{CreateBuildRequest, CreateCommitsRequest, PublishRepoPushRequest},
    error::{BuildError, NotFoundError},
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::ProcessPostReceiveServerRequest,
};

#[axum::debug_handler]
pub async fn process_post_receive(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<ProcessPostReceiveServerRequest>,
) -> Result<AppResponse<()>, AppError> {
    let commit_request = CreateCommitsRequest::new(
        &owner,
        &repo,
        request.old_sha.clone(),
        request.new_sha.clone(),
        request.ref_name.clone(),
        None,
        Default::default(),
    )?;
    let build_request = CreateBuildRequest::new(
        &owner,
        &repo,
        request.ref_name.clone(),
        request.new_sha.clone(),
    )?;

    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").ok();
    let owner_slack = owner.clone();
    let repo_slack = repo.clone();
    let ref_name_slack = request.ref_name.clone();
    let publish_request = PublishRepoPushRequest {
        owner: owner.clone(),
        repo: repo.clone(),
        ref_name: request.ref_name.clone(),
        old_sha: request.old_sha.clone(),
        new_sha: request.new_sha.clone(),
        pusher_id: request.pusher_id,
    };

    // execute in the background to avoid blocking push operation
    tokio::spawn(async move {
        if let Err(e) = state.webhook_service.publish_repo_push(publish_request).await {
            tracing::error!("Failed to publish repo push event: {e}");
        }

        let commits = match state.commit_service.create_commits(commit_request).await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to create commits in post-receive: {e}");
                return;
            }
        };

        let Some(webhook_url) = slack_webhook_url else {
            return;
        };

        let branch = ref_name_slack
            .strip_prefix("refs/heads/")
            .unwrap_or(&ref_name_slack);

        let text = if commits.is_empty() {
            format!(
                "https://www.gitdot.io/{}/{}/commits/{}",
                owner_slack, repo_slack, branch
            )
        } else {
            let count_str = if commits.len() == 1 {
                "1 new commit".to_string()
            } else {
                format!("{} new commits", commits.len())
            };
            let commit_lines: Vec<String> = commits
                .iter()
                .map(|c| {
                    let short_sha = c.sha.chars().take(7).collect::<String>();
                    let first_line = c.message.lines().next().unwrap_or("").to_string();
                    let url = format!(
                        "https://www.gitdot.io/{}/{}/commits/{}",
                        owner_slack, repo_slack, short_sha
                    );
                    format!("<{}|{}> - {}", url, short_sha, first_line)
                })
                .collect();
            format!(
                "{} pushed to `{}` by {}\n{}\n{}/{}",
                count_str,
                branch,
                owner_slack,
                commit_lines.join("\n"),
                owner_slack,
                repo_slack
            )
        };

        let body = json!({ "text": text }).to_string();
        let client = Client::new();
        if let Err(e) = client
            .post(webhook_url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
        {
            tracing::error!("Failed to send Slack notification: {e}");
        }
    });
    tokio::spawn(async move {
        if let Err(e) = state.build_service.create_build(build_request).await {
            if !matches!(
                e,
                BuildError::NotFound(NotFoundError {
                    entity: "config",
                    ..
                })
            ) {
                tracing::error!("Failed to create build in post-receive: {e}");
            }
        }
    });

    Ok(AppResponse::new(StatusCode::OK, ()))
}
